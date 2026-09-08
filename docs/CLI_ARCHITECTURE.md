# Agent-Devex CLI architecture and implementation roadmap

This document describes the current `agent-devex` binary (crate root at the repository root) and the incremental work to add structured `agent.toml` configuration plus `compile` and `run` command stubs.

## Binary and library split

| Crate surface | Role |
| --- | --- |
| `src/main.rs` | Clap parsing, tracing, `human-panic`, dotenv, command dispatch |
| `src/lib.rs` | Shared modules used by the binary and integration tests |
| `src/commands/` | Per-command handlers (`init`, `deploy`, `validate`, `doctor`, `status`) |

The binary uses `anyhow::Result` at the process edge. Domain failures use `thiserror` via `AgentDevexError` and convert with `Into::into`.

## Existing commands

- **init** — Scaffold `contracts/agent_pay_integration` and `agent/` (TypeScript or Python MCP) from `templates/` with `{{PROJECT_NAME}}` substitution. Writes `agent-devex.toml` today.
- **deploy** — Shells out to `stellar contract build`, then `stellar contract deploy` on testnet when `STELLAR_ACCOUNT` is set. Persists ids under `.agent-devex/state.toml`.
- **validate** — Layout and env-template checks for a generated project.
- **doctor** — Host toolchain diagnostics.
- **status** — Local network / language / contract id from config and state.

## Configuration today

Tool-level optional config is `agent-devex.toml` (`AgentDevexConfig`: network, default MCP language, project metadata). Deployed contract identity lives in local project state, not in secrets files.

## Roadmap (this series)

1. Typed errors covering config parse/serialize, invalid network/port, and project creation.
2. Keep `human-panic` for unexpected crashes; keep `Result` for expected CLI failures.
3. Introduce `AgentConfig` and project `agent.toml` (network, contract ids, MCP, port) with load/save.
4. Add **compile** (load config, log Soroban compile stub) and **run** (MCP port resolution stub).
5. Route new commands from `main` without panicking on recoverable errors.

Templates remain on disk under `templates/` and are embedded with `include_str!`. AgentPay / AgentGuard in generated contracts stay local interfaces.
