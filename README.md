# Agent-Devex

CLI to scaffold, test, and deploy AI-to-Web3 integrations on Stellar: a Soroban contract
pre-wired with AgentPay/AgentGuard interfaces and an MCP server stub (`settle_and_execute`).

```bash
cargo run -- init my-agent --lang ts
cargo run -- init my-agent --lang py
cargo run -- deploy --project-dir my-agent --network testnet
```

Requires [stellar-cli](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) on `PATH` for `deploy`. Set `STELLAR_ACCOUNT` to run deploy instead of printing the command.


The repo currently contains only LICENSE. This plan adds a single Cargo binary crate plus embedded templates. No unpublished AgentPay crates exist, so the generated contract will define local AgentPay / AgentGuard interfaces (traits + types) that a later crate can replace.

Layout

Cargo.toml                 # clap, anyhow, thiserror (optional)
src/main.rs                # clap CLI + command dispatch
src/scaffold.rs            # mkdir + write template files
templates/
  contracts/agent_pay_integration/{Cargo.toml,src/lib.rs}
  agent/ts/{package.json,tsconfig.json,src/index.ts,README.md}
  agent/py/{pyproject.toml,src/server.py,README.md}
  project/README.md        # generated root README

init writes:

<project-name>/
  contracts/agent_pay_integration/
  agent/                   # TS or Py MCP server
  README.md

File generation uses include_str!("../templates/...") so templates stay editable on disk and ship inside the binary. Direct string literals are avoided except for tiny generated snippets (project name substitution). Substitution is a simple {{PROJECT_NAME}} / {{CONTRACT_ID}} replace after include_str!. Comments in main.rs / scaffold.rs will document this vs writing large strings in Rust.

Part 1 — CLI (src/main.rs)

clap derive API:





Binary name: agent-devex



init <project-name> --lang ts|py (enum Lang { Ts, Py })



deploy [--project-dir .] [--network testnet] stub

Behavior:





init: refuse if target dir exists and is non-empty; create contracts/ + agent/; write templates; print next steps (cd, stellar contract build, npm install / uv sync).



deploy: anyhow errors if contracts/agent_pay_integration is missing. Invoke:





stellar contract build (cwd = contract crate)



stellar contract deploy --network testnet --source-account $STELLAR_ACCOUNT --wasm <built wasm> (or print the exact command if STELLAR_ACCOUNT is unset)

Do not invent a fake deploy success; surface command output and fail on non-zero exit (std::process::Command).

Error handling: fn main() -> anyhow::Result<()> and ? throughout.

Part 2 — Soroban template

[templates/contracts/agent_pay_integration/src/lib.rs](templates/contracts/agent_pay_integration/src/lib.rs) (copied to generated projects as contracts/agent_pay_integration/src/lib.rs):





soroban-sdk contract with #![no_std]



Modules or traits agent_pay and agent_guard:





AgentGuard::assert_authorized(env, agent) — dummy identity check (e.g. require agent in a stored allowlist)



AgentPay::settle(env, payer, amount) — dummy payment settlement (increment a paid counter / require amount > 0)



Public execute_agent_action(agent, action_id, amount) that: verify → settle → then mutate state (e.g. store last action_id)



Matching [Cargo.toml](templates/contracts/agent_pay_integration/Cargo.toml) with crate-type = ["cdylib"] and current soroban-sdk (0.23-compatible with stellar-cli; pin a recent stable in the template)

Comments in the contract will state that AgentPay/AgentGuard are local interfaces standing in for a future shared crate.

Part 3 — MCP stubs

TypeScript (--lang ts)

[templates/agent/ts/src/index.ts](templates/agent/ts/src/index.ts):





@modelcontextprotocol/sdk stdio Server + ListTools / CallTool



Tool settle_and_execute with args: prompt, agent_address, action_id, amount (and optional contract_id)



Comments describing the LLM → MCP tool → Stellar tx bridge



Boilerplate: TransactionBuilder + Contract.call("execute_agent_action", ...) via @stellar/stellar-sdk, sign with Keypair.fromSecret(process.env.STELLAR_SECRET_KEY), submit to Horizon/Soroban RPC testnet



Keep network/RPC URLs as env (STELLAR_RPC_URL, STELLAR_NETWORK_PASSPHRASE, AGENTPAY_CONTRACT_ID) so the stub is copy-paste runnable after init



package.json: @modelcontextprotocol/sdk, @stellar/stellar-sdk, tsx/typescript as needed

Python (--lang py)

Mirror the same tool and env vars using the official mcp Python SDK + stellar-sdk: [templates/agent/py/src/server.py](templates/agent/py/src/server.py) with pyproject.toml dependencies.

Deploy vs MCP

sequenceDiagram
  participant LLM
  participant MCP as MCP_Server
  participant Horizon as Stellar_RPC
  participant Contract as AgentPay_Contract

  LLM->>MCP: tools/call settle_and_execute
  MCP->>MCP: map prompt args to Soroban invoke
  MCP->>Horizon: signed tx execute_agent_action
  Horizon->>Contract: verify AgentGuard then AgentPay.settle
  Contract-->>Horizon: state updated
  Horizon-->>MCP: result
  MCP-->>LLM: tool result text

Out of scope





Real on-chain AgentPay protocol / published crates



Tests for generated projects



Publishing the CLI to crates.io
