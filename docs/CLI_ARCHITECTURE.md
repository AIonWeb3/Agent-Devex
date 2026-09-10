# Agent-Devex CLI architecture

This document describes the `agent-devex` binary (crate root at the repository root).

## Binary and library split

| Crate surface | Role |
| --- | --- |
| `src/main.rs` | Clap parsing, tracing, panic hook, dotenv, command dispatch, error exit |
| `src/lib.rs` | Shared modules used by the binary and integration tests |
| `src/commands/` | Handlers: `init`, `compile`, `run`, `deploy`, `validate`, `doctor`, `status` |
| `src/errors.rs` | `AgentDevexError` + `Result` alias |
| `src/config.rs` | `AgentDevexConfig` (`agent-devex.toml`) and `AgentConfig` (`agent.toml`) |
| `src/panic.rs` | `human-panic` metadata and hook install |

Domain failures use `thiserror` (`AgentDevexError`). `main` reports `anyhow` errors with `tracing::error` and exits `1`. Unexpected panics use `human-panic` (release dumps).

## Commands

- **init** — Scaffold `contracts/agent_pay_integration` and `agent/` from `templates/`. Writes `agent-devex.toml` and default `agent.toml`. Fails with `ProjectAlreadyExists` if the directory is non-empty.
- **compile** — Loads `agent.toml`, logs `Compiling Soroban contracts...`. Does **not** invoke `stellar contract build` (stub).
- **run** — Resolves MCP port (`--port`, else `agent.toml`, else `3000`). Logs local MCP init. Does **not** bind a socket (stub).
- **deploy** — Validates `testnet` \| `mainnet`, then `stellar contract build` and optional `stellar contract deploy` when `STELLAR_ACCOUNT` is set. Persists ids under `.agent-devex/state.toml`.
- **validate** / **doctor** / **status** — Unchanged project health, toolchain, and local state commands.

## Configuration

- Tool-level optional `agent-devex.toml` (`AgentDevexConfig`).
- Project-level `agent.toml` (`AgentConfig`: network, contract ids, MCP lang/port). Defaults: `testnet`, no contract ids, port `3000`. No private keys.

Templates stay under `templates/` and are embedded with `include_str!`. AgentPay / AgentGuard in generated contracts remain local interfaces.
