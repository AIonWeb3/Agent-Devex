# Agent-Devex

CLI for scaffolding **AI-to-Web3** projects on [Stellar](https://stellar.org): a [Soroban](https://developers.stellar.org/docs/build/smart-contracts) contract pre-wired with **AgentPay** / **AgentGuard** interfaces, plus a [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server so an LLM can call `settle_and_execute` and submit a signed contract invoke.

```text
LLM  →  MCP tool settle_and_execute  →  signed Soroban tx  →  execute_agent_action
                                                      (AgentGuard → AgentPay.settle → state)
```

## Requirements

- [Rust](https://rustup.rs) (stable; this crate uses edition 2024)
- For `deploy`: [stellar-cli](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) on `PATH`, plus the `wasm32-unknown-unknown` target
- For a generated TypeScript agent: Node.js and npm
- For a generated Python agent: Python 3.11+ and [uv](https://docs.astral.sh/uv/) (or any installer that reads `pyproject.toml`)

## Install / run

From this repository:

```bash
cargo build --release
cargo run -- --help
```

Install the binary onto your `PATH`:

```bash
cargo install --path .
agent-devex --help
```

## Commands

### `init`

Create a monorepo. The target directory must not exist, or must be empty.

```bash
agent-devex init my-agent --lang ts   # TypeScript MCP server
agent-devex init my-agent --lang py   # Python MCP server
```

Layout:

```text
my-agent/
  README.md
  contracts/agent_pay_integration/   # Soroban crate (cdylib)
  agent/                             # MCP stdio server
```

Templates live under [`templates/`](templates/) in this repo and are compiled into the CLI with `include_str!`. At `init` time they are written to disk; `{{PROJECT_NAME}}` is substituted in READMEs and package names.

### `deploy`

Build the Soroban contract with `stellar contract build`, then deploy to a Stellar network (default: **testnet**).

```bash
agent-devex deploy --project-dir my-agent --network testnet
```

Set `STELLAR_ACCOUNT` to the source account stellar-cli should use. If it is unset, the CLI still builds and prints the `stellar contract deploy ...` command instead of submitting.

`deploy` fails if `contracts/agent_pay_integration/Cargo.toml` is missing, if `stellar` is not on `PATH`, or if the child process exits non-zero.

## Generated contract

`contracts/agent_pay_integration` is a `#![no_std]` Soroban contract (`soroban-sdk` 22). **AgentPay** and **AgentGuard** are local interfaces, not a published protocol crate.

| Entry | Role |
|--------|------|
| `allow_agent` | Admin auth; add an agent to the allowlist |
| `execute_agent_action` | `require_auth` + allowlist, dummy settle (`amount > 0`), then store `action_id` |
| `last_action` / `paid` | Read last action and accumulated dummy payment |

Treat settlement as a teaching stub, not token transfer or production payment security.

## Generated MCP server

Both languages expose one tool: **`settle_and_execute`**.

| Argument | Meaning |
|----------|---------|
| `prompt` | LLM intent (logged in the tool result; not executed as code) |
| `agent_address` | Agent’s Stellar/Soroban address |
| `action_id` | Passed to the contract as a symbol |
| `amount` | i128 amount (string) |
| `contract_id` | Optional override for `AGENTPAY_CONTRACT_ID` |

Environment:

| Variable | Required | Default |
|----------|----------|---------|
| `STELLAR_SECRET_KEY` | yes | — |
| `AGENTPAY_CONTRACT_ID` | yes (unless `contract_id` is passed) | — |
| `STELLAR_RPC_URL` | no | `https://soroban-testnet.stellar.org` |
| `STELLAR_NETWORK_PASSPHRASE` | no | Test SDF Network ; September 2015 |

The server loads the source account from Soroban RPC, builds an invoke of `execute_agent_action`, prepares and **signs** the transaction, then submits it.

TypeScript (`--lang ts`):

```bash
cd my-agent/agent
npm install
npx tsx src/index.ts
```

Python (`--lang py`):

```bash
cd my-agent/agent
uv sync
uv run python src/server.py
```

Point your MCP-capable client (Cursor, Claude Desktop, etc.) at that stdio process.

## Repository layout (this CLI)

| Path | Purpose |
|------|---------|
| [`src/main.rs`](src/main.rs) | clap CLI (`init`, `deploy`) |
| [`src/scaffold.rs`](src/scaffold.rs) | Write templates into a new project |
| [`templates/contracts/`](templates/contracts/) | Soroban AgentPay integration crate |
| [`templates/agent/ts/`](templates/agent/ts/) | MCP server (TypeScript) |
| [`templates/agent/py/`](templates/agent/py/) | MCP server (Python) |
| [`templates/project/README.md`](templates/project/README.md) | README copied into generated repos |

## Status

Early architecture: scaffolding and a deploy stub. There is no published AgentPay crate, no crates.io release, and no automated test suite for generated projects yet.

## License

[MIT](LICENSE) © AIonWeb3
