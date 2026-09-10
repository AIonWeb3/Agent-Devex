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

Agent-Devex bridges the gap between **AI agents** and **blockchain actions**. It's a CLI that generates a complete project — a Soroban smart contract pre-wired with payment settlement and access control, plus an MCP server that lets any LLM submit signed transactions to Stellar.

```
LLM  →  MCP tool settle_and_execute  →  signed Soroban tx  →  execute_agent_action
                                              (AgentGuard → AgentPay.settle → state)
```

### The Problem

Building AI agents that can interact with blockchain requires deep knowledge of smart contract development, transaction signing, blockchain RPC, and the Model Context Protocol. Developers spend weeks on boilerplate before writing any business logic.

### The Solution

One command generates everything you need:

```bash
agent-devex init my-agent --lang ts
```

You get a fully functional monorepo with a battle-tested smart contract and a ready-to-run MCP server — start building your AI×Web3 product in minutes, not weeks.

---

## Features

| Feature | Description |
|---------|-------------|
| **Project Scaffolding** | Generate a complete monorepo with one command |
| **Smart Contract** | Soroban contract with AgentPay settlement + AgentGuard access control |
| **MCP Server** | TypeScript, Python, or Rust MCP server for LLM integration |
| **One-Click Deploy** | Build and deploy contracts to Stellar testnet/mainnet |
| **Project Validation** | Verify project structure and configuration health |
| **Toolchain Doctor** | Diagnose missing dependencies and provide fix instructions |
| **Contract Status** | Query deployed contract state from the terminal |
| **Interactive Mode** | Guided project setup with smart defaults |
| **Security First** | Secrets handling, `.env` templates, access control patterns |

---

## Quick Start

### Prerequisites

| Tool | Required | Purpose |
|------|----------|---------|
| **Rust** (stable 1.85+) | Yes | Build the CLI |
| **stellar-cli** | For deploy | Compile and deploy Soroban contracts |
| **wasm32-unknown-unknown** | For deploy | Rust target for WebAssembly compilation |
| **Node.js + npm** | For TS agent | Run TypeScript MCP server |
| **Python 3.11+ + uv** | For PY agent | Run Python MCP server |

> **Tip:** Run `agent-devex doctor` after installation to verify your toolchain.

### Install

```bash
# From source
cargo install --path .

# Or build and run directly
cargo build --release
./target/release/agent-devex --help
```

### Create Your First Project

```bash
# Scaffold a new project with TypeScript MCP server
agent-devex init my-agent --lang ts

# Or with Python
agent-devex init my-agent --lang py

# Or launch interactive mode (auto-detected when --lang is omitted on a TTY)
agent-devex init my-agent
```

### Project Structure

```
my-agent/
├── README.md                              # Project documentation
├── agent-devex.toml                       # CLI configuration
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
    └── src/
        └── index.ts / server.py           # MCP server implementation
```

### Build & Deploy the Contract

```bash
cd my-agent

# Build the Soroban WASM
stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml

# Deploy to testnet (set your source account first)
export STELLAR_ACCOUNT=your-account-name
agent-devex deploy --project-dir . --network testnet
```

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

Point your MCP-capable client (Cursor, Claude Desktop, VS Code, etc.) at the stdio process and the LLM can now call `settle_and_execute`.

---

## Commands

### `init` — Scaffold a New Project

```bash
agent-devex init <project-name> --lang <ts|py|rs>
```

Creates a monorepo with a Soroban contract and MCP server. The target directory must not exist or must be empty.

### `deploy` — Build & Deploy Contract

```bash
agent-devex deploy --project-dir <path> --network <network>
```

Compiles the Soroban contract and deploys it. Set `STELLAR_ACCOUNT` for automated deployment.

### `validate` — Check Project Health

```bash
agent-devex validate --project-dir <path>
```

Verifies project structure, configuration files, and template integrity.

### `doctor` — Diagnose Toolchain

```bash
agent-devex doctor
```

Checks for required and optional tools, reports versions, and provides installation instructions for anything missing.

### `status` — Query Contract State

```bash
agent-devex status --project-dir <path>
```

Queries the deployed contract for last action and payment totals.

---

## Architecture

### Smart Contract (Soroban)

The generated contract implements two local interfaces:

| Interface | Purpose |
|-----------|---------|
| **AgentGuard** | Access control — maintains an allowlist of authorized agent addresses |
| **AgentPay** | Payment settlement — verifies positive amounts and tracks cumulative payments |

**Contract Entry Points:**

| Function | Description |
|----------|-------------|
| `allow_agent(admin, agent)` | Admin adds an agent to the allowlist |
| `execute_agent_action(agent, action_id, amount)` | Verify auth → settle payment → record action |
| `last_action()` | Read the most recent action ID |
| `paid(agent)` | Read accumulated payment for an agent |

### MCP Server

Both TypeScript and Python servers expose one primary tool:

**`settle_and_execute`**

| Parameter | Type | Description |
|-----------|------|-------------|
| `prompt` | string | LLM intent (logged, not executed) |
| `agent_address` | string | Agent's Stellar/Soroban address |
| `action_id` | string | Symbol-like action identifier |
| `amount` | string | i128 payment amount (decimal string) |
| `contract_id` | string? | Override for `AGENTPAY_CONTRACT_ID` |

**Environment Variables:**

| Variable | Required | Default |
|----------|----------|---------|
| `STELLAR_SECRET_KEY` | Yes | — |
| `AGENTPAY_CONTRACT_ID` | Yes* | — |
| `STELLAR_RPC_URL` | No | `https://soroban-testnet.stellar.org` |
| `STELLAR_NETWORK_PASSPHRASE` | No | `Test SDF Network ; September 2015` |

\* Not required if `contract_id` is passed per-call.

---

## Configuration

### `agent-devex.toml`

Optional project-level configuration:

```toml
# Default Stellar network for deploy (overridden by --network)
network = "testnet"

# Default MCP server language
default_lang = "ts"

# Project metadata
[project]
author = "Your Name"
description = "My AI×Web3 integration"
```

---

## Repository Layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | CLI entry point and command dispatch |
| `src/lib.rs` | Library root with public API |
| `src/commands/` | Command implementations (init, deploy, validate, doctor, status) |
| `src/scaffold.rs` | Template file generation engine |
| `src/config.rs` | Configuration loading and validation |
| `src/errors.rs` | Typed error definitions |
| `templates/contracts/` | Soroban AgentPay integration crate template |
| `templates/agent/ts/` | TypeScript MCP server template |
| `templates/agent/py/` | Python MCP server template |
| `templates/project/` | Project-level file templates |
| `dashboard/` | Static web dashboard for demos and documentation |
| `tests/` | Integration tests |

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Development workflow
cargo check                        # Type check
cargo test                         # Run tests
cargo clippy --all-targets         # Lint
cargo fmt --all                    # Format
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
- [x] GitHub Actions CI (fmt, clippy, tests)
- [x] Static pitch dashboard
- [ ] Published AgentPay crate (replace local interfaces)
- [ ] Template update/migration system
- [ ] Plugin system for custom contract interfaces
- [ ] Multi-contract project support
- [ ] Audited mainnet deployment guides

## More documentation

| Doc | Purpose |
|-----|---------|
| [docs/DEMO.md](docs/DEMO.md) | Client walkthrough |
| [docs/DEPLOY.md](docs/DEPLOY.md) | Binary, contract, MCP hosting |
| [docs/PITCH.md](docs/PITCH.md) | Problem / solution / buyers |
| [dashboard/](dashboard/) | Visual pitch site |

## Known limitations

- AgentPay / AgentGuard in generated contracts are **local interfaces**, not a published protocol.
- `deploy` requires `stellar-cli` and a funded account; without `STELLAR_ACCOUNT` it prints the command after build.
- Dashboard MCP form does **not** submit chain transactions (validation only).
- Secret keys must never be committed; use `.env` locally.

---

<p align="center">
  <strong>Built with ❤️ by <a href="https://github.com/AIonWeb3">AIonWeb3</a></strong><br/>
  <sub>Bridging AI agents and Web3 on Stellar</sub>
</p>
