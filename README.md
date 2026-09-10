<p align="center">
  <h1 align="center">Agent-Devex</h1>
  <p align="center">
    <strong>The Developer Experience toolkit for AI-to-Web3 integrations on Stellar</strong>
  </p>
  <p align="center">
    Scaffold production-ready Soroban smart contracts with MCP servers<br/>
    so any LLM can invoke on-chain actions in seconds.
  </p>
</p>

<p align="center">
  <a href="https://github.com/AIonWeb3/Agent-Devex/actions/workflows/rust.yml">
    <img src="https://github.com/AIonWeb3/Agent-Devex/actions/workflows/rust.yml/badge.svg" alt="CI Status" />
  </a>
  <a href="https://github.com/AIonWeb3/Agent-Devex/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT" />
  </a>
  <a href="https://github.com/AIonWeb3/Agent-Devex/releases">
    <img src="https://img.shields.io/github/v/release/AIonWeb3/Agent-Devex?include_prereleases&label=version" alt="Version" />
  </a>
  <img src="https://img.shields.io/badge/rust-1.85+-orange.svg" alt="Rust 1.85+" />
  <img src="https://img.shields.io/badge/stellar-soroban-purple.svg" alt="Stellar Soroban" />
</p>

---

## What is Agent-Devex?

Agent-Devex is the **`agent-devex`** Cargo binary (crate root at this repository root). It scaffolds a complete project: a Soroban smart contract pre-wired with payment settlement and access control, plus an MCP server so an LLM can submit signed transactions to Stellar.

```
LLM  →  MCP tool settle_and_execute  →  signed Soroban tx  →  execute_agent_action
                                              (AgentGuard → AgentPay.settle → state)
```

### The Problem

Building AI agents that can interact with blockchain requires smart contract development, transaction signing, RPC, and the Model Context Protocol. Developers spend weeks on boilerplate before writing business logic.

### The Solution

One command generates the monorepo:

```bash
agent-devex init my-agent --lang ts
```

You get a Soroban contract (local AgentPay / AgentGuard interfaces) and a TypeScript or Python MCP server. Templates live under `templates/` and are compiled into the binary with `include_str!` plus `{{PROJECT_NAME}}` / `{{DEFAULT_LANG}}` substitution.

---

## Features

| Feature | Description |
|---------|-------------|
| **Project scaffolding** | `init` writes `contracts/agent_pay_integration`, `agent/` (TS or Python), `agent.toml`, and `agent-devex.toml` |
| **Smart contract** | Soroban contract with local AgentPay settlement and AgentGuard access control |
| **MCP server** | TypeScript or Python MCP server; tool is `settle_and_execute` (prompt is logged, not executed) |
| **agent.toml** | Project config for network, contract ids, and MCP lang/port (no private keys) |
| **Deploy** | `stellar contract build`, then testnet/mainnet deploy when `STELLAR_ACCOUNT` is set |
| **Compile stub** | Loads `agent.toml` and logs the Soroban compile step (does not invoke `stellar contract build`) |
| **Run stub** | Resolves MCP port and logs local MCP init (does not bind a socket) |
| **Project validation** | Verify generated layout and configuration |
| **Toolchain doctor** | Diagnose missing dependencies |
| **Contract status** | Show local network, language, and contract id (`--require-id` for CI) |
| **Interactive init** | Language prompt on a TTY when `--lang` is omitted |
| **Security** | `.env` templates, dotenv load without logging secrets, access-control patterns |
| **Errors and panics** | `thiserror` (`AgentDevexError`) in the library; `anyhow` at `main`; `human-panic` for unexpected crashes |

---

## Quick Start

### Prerequisites

| Tool | Required | Purpose |
|------|----------|---------|
| **Rust** (stable 1.85+) | Yes | Build the CLI (`rust-toolchain.toml` also adds rustfmt, clippy, `wasm32-unknown-unknown`) |
| **stellar-cli** | For deploy | Compile and deploy Soroban contracts |
| **wasm32-unknown-unknown** | For deploy | Rust target for WebAssembly |
| **Node.js + npm** | For TS agent | Run the TypeScript MCP server |
| **Python 3.11+ + uv** | For PY agent | Run the Python MCP server |

> **Tip:** Run `agent-devex doctor` after installation to verify your toolchain.

### Install

```bash
# From source (do not run `cargo new agent-devex` inside this repo)
cargo install --path .

# Or build and run directly
cargo build --release
./target/release/agent-devex --help
```

On Windows the binary is `target/release/agent-devex.exe`.

### Create Your First Project

```bash
# Scaffold a new project with TypeScript MCP server
agent-devex init my-agent --lang ts

# Or with Python
agent-devex init my-agent --lang py

# Interactive language choice when --lang is omitted on a TTY
agent-devex init my-agent
```

`init` fails with `ProjectAlreadyExists` if the target directory exists and is not empty. Non-TTY sessions without `--lang` default to TypeScript (or `default_lang` from `agent-devex.toml` in the current directory, if set).

### Project Structure

```
my-agent/
├── README.md                              # Project documentation
├── agent.toml                             # Project config (network, MCP, contract ids)
├── agent-devex.toml                       # Tool defaults (network, default_lang, metadata)
├── .env.example                           # Environment variable template
├── .gitignore                             # Git ignore rules
├── contracts/
│   └── agent_pay_integration/
│       ├── Cargo.toml                     # Soroban contract manifest
│       └── src/
│           └── lib.rs                     # AgentPay + AgentGuard contract
└── agent/
    ├── README.md                          # Agent setup guide
    ├── package.json / pyproject.toml      # Dependencies
    ├── tsconfig.json                      # TypeScript only
    └── src/
        └── index.ts / server.py           # MCP server implementation
```

### Compile, Deploy, and Run (CLI)

```bash
cd my-agent

# Load agent.toml and log the compile workflow (does not run stellar contract build yet)
agent-devex compile --project-dir .

# Build WASM and deploy to testnet when STELLAR_ACCOUNT is set
export STELLAR_ACCOUNT=your-account-name
agent-devex deploy --project-dir . --network testnet

# Resolve MCP port (--port, else agent.toml, else 3000) and log init (does not start MCP)
agent-devex run --project-dir . --port 3000
```

`compile` and `run` are stubs: they validate configuration and log progress. WASM compilation happens inside `deploy` via `stellar contract build`. Contract ids are stored under `.agent-devex/state.toml`. To serve MCP today, run the generated TypeScript or Python process below.

Process environment always wins. The CLI loads `.env` for missing keys only (never logged). For `compile`, `run`, `deploy`, `validate`, and `status`, a project-dir `.env` is also loaded if present.

### Run the MCP Server

**TypeScript:**
```bash
cd my-agent/agent
npm install
export STELLAR_SECRET_KEY=S...
export AGENTPAY_CONTRACT_ID=C...
npx tsx src/index.ts
```

**Python:**
```bash
cd my-agent/agent
uv sync
export STELLAR_SECRET_KEY=S...
export AGENTPAY_CONTRACT_ID=C...
uv run python src/server.py
```

Point your MCP-capable client (Cursor, Claude Desktop, VS Code, etc.) at the stdio process. The LLM can call `settle_and_execute`. Optional RPC env: `STELLAR_RPC_URL`, `STELLAR_NETWORK_PASSPHRASE`.

---

## Commands

Binary name: **`agent-devex`**. Recoverable failures print via tracing and exit `1`; they do not panic. Set `NO_COLOR` for plain log output.

### `init` — Scaffold a New Project

```bash
agent-devex init <PROJECT_NAME> [--lang ts|py]
```

Creates a monorepo with a Soroban contract and MCP server, writes `agent.toml` (defaults: network `testnet`, MCP port `3000`, `mcp.lang` from `--lang`) and `agent-devex.toml`.

### `compile` — Compile workflow (stub)

```bash
agent-devex compile [--project-dir <path>]
```

Loads `agent.toml` and logs `Compiling Soroban contracts...`. Does **not** invoke `stellar contract build`.

### `deploy` — Build and Deploy Contract

```bash
agent-devex deploy [--project-dir <path>] [--network testnet|mainnet]
```

Accepts only `testnet` or `mainnet`. Builds via `stellar contract build`, then deploys when `STELLAR_ACCOUNT` is set. Without that account, it prints next steps after a successful build.

### `run` — Local MCP init (stub)

```bash
agent-devex run [--project-dir <path>] [--port <u16>]
```

Port order: `--port`, then `agent.toml`, then `3000`. Port `0` is rejected. Logs MCP initialization; does **not** bind a socket or start the generated MCP process.

### `validate` — Check Project Health

```bash
agent-devex validate [--project-dir <path>]
```

Verifies project structure, configuration files, and template integrity.

### `doctor` — Diagnose Toolchain

```bash
agent-devex doctor
```

Checks for required and optional tools, reports versions, and provides installation instructions for anything missing.

### `status` — Local contract configuration

```bash
agent-devex status [--project-dir <path>] [--require-id]
```

Shows local network, language, RPC, and contract id from config, `.agent-devex/state.toml`, and `AGENTPAY_CONTRACT_ID`. `--require-id` exits with an error when no contract id is configured.

---

## Architecture

### Smart Contract (Soroban)

The generated contract implements two **local** interfaces (not published crates):

| Interface | Purpose |
|-----------|---------|
| **AgentGuard** | Access control — allowlist of authorized agent addresses |
| **AgentPay** | Payment settlement — positive amounts and cumulative payments |

**Contract entry points:**

| Function | Description |
|----------|-------------|
| `allow_agent(admin, agent)` | Admin adds an agent to the allowlist |
| `execute_agent_action(agent, action_id, amount)` | Verify auth, settle payment, record action |
| `last_action()` | Read the most recent action ID |
| `paid(agent)` | Read accumulated payment for an agent |

### MCP Server

TypeScript and Python servers expose one primary tool:

**`settle_and_execute`**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prompt` | string | LLM intent (logged, not executed) |
| `agent_address` | string | Agent's Stellar/Soroban address |
| `action_id` | string | Symbol-like action identifier |
| `amount` | string | i128 payment amount (decimal string) |
| `contract_id` | string? | Override for `AGENTPAY_CONTRACT_ID` |

**Environment variables:**

| Variable | Required | Default |
|----------|----------|---------|
| `STELLAR_SECRET_KEY` | Yes | — |
| `AGENTPAY_CONTRACT_ID` | Yes* | — |
| `STELLAR_RPC_URL` | No | `https://soroban-testnet.stellar.org` |
| `STELLAR_NETWORK_PASSPHRASE` | No | `Test SDF Network ; September 2015` |

\* Not required if `contract_id` is passed per-call.

---

## Configuration

Neither `agent.toml` nor `agent-devex.toml` stores private keys. Use `.env` / `.env.example` for secrets.

### `agent.toml`

Written by `init`. Used by `compile` and `run` (and as the project-level schema for network, contract ids, and MCP).

```toml
network = "testnet"

[contract_ids]
# agent_pay = "C..."   # optional; omit until deployed

[mcp]
lang = "ts"
port = 3000
```

Defaults: `testnet`, no contract ids, port `3000`.

### `agent-devex.toml`

Optional tool defaults (also generated on `init`):

```toml
# Default Stellar network for deploy (overridden by --network)
network = "testnet"

# Default MCP server language when init omits --lang
default_lang = "ts"

# Project metadata
[project]
author = "Your Name"
description = "My AI x Web3 integration"
```

---

## Repository Layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | CLI entry: Clap, tracing, panic hook, dotenv, dispatch, error exit |
| `src/lib.rs` | Library modules used by the binary and integration tests |
| `src/commands/` | `init`, `compile`, `run`, `deploy`, `validate`, `doctor`, `status` |
| `src/scaffold.rs` | Template generation (`include_str!`) |
| `src/config.rs` | `AgentConfig` (`agent.toml`) and `AgentDevexConfig` (`agent-devex.toml`) |
| `src/errors.rs` | Typed `AgentDevexError` |
| `src/panic.rs` | `human-panic` metadata and hook |
| `src/state.rs` | Local `.agent-devex/state.toml` after deploy |
| `templates/` | Soroban contract, TS/Python MCP, and project file templates |
| `examples/demo-agent/` | Sample generated-style project |
| `dashboard/` | Static web dashboard for demos |
| `tests/` | Integration tests |
| `.github/workflows/rust.yml` | CI: fmt, clippy, tests |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
cargo check
cargo test
cargo clippy --all-targets
cargo fmt --all
```

---

## License

MIT © [AIonWeb3](https://github.com/AIonWeb3)

---

## Roadmap

- [x] CLI scaffolding (`init` + `deploy`)
- [x] TypeScript and Python MCP server templates
- [x] `validate`, `doctor`, `status`
- [x] Interactive `init` and project config
- [x] `agent.toml` load/save and typed CLI errors
- [x] `compile` and `run` command stubs
- [x] Human-friendly panic reports (`human-panic`)
- [x] GitHub Actions CI (fmt, clippy, tests)
- [x] Static pitch dashboard
- [ ] Implement `compile` (`stellar contract build`) and `run` (start MCP)
- [ ] Published AgentPay crate (replace local interfaces)
- [ ] Template update/migration system
- [ ] Plugin system for custom contract interfaces
- [ ] Multi-contract project support
- [ ] Audited mainnet deployment guides

## More documentation

| Doc | Purpose |
|-----|---------|
| [docs/CLI_ARCHITECTURE.md](docs/CLI_ARCHITECTURE.md) | CLI modules, commands, config |
| [docs/ERROR_HANDLING.md](docs/ERROR_HANDLING.md) | Error flow (`anyhow` vs `AgentDevexError`) |
| [docs/DEMO.md](docs/DEMO.md) | Client walkthrough |
| [docs/DEPLOY.md](docs/DEPLOY.md) | Binary, contract, MCP hosting |
| [docs/PITCH.md](docs/PITCH.md) | Problem / solution / buyers |
| [dashboard/](dashboard/) | Visual pitch site |

## Known limitations

- AgentPay / AgentGuard in generated contracts are **local interfaces**, not a published protocol.
- `compile` does not run Soroban compilation; `run` does not start an MCP server.
- MCP languages are `ts` and `py` only (no Rust MCP template).
- `deploy --network` accepts only `testnet` or `mainnet`.
- `deploy` requires `stellar-cli` and a funded account; without `STELLAR_ACCOUNT` it prints the command after build.
- Dashboard MCP form does **not** submit chain transactions (validation only).
- Secret keys must never be committed; use `.env` locally.

---

<p align="center">
  <strong>Built by <a href="https://github.com/AIonWeb3">AIonWeb3</a></strong><br/>
  <sub>Bridging AI agents and Web3 on Stellar</sub>
</p>
