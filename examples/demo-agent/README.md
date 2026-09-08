# Demo agent (presentation fixture)

This folder documents a **realistic client walkthrough** without embedding secrets.

## Story

A marketplace wants an LLM agent that can settle a small on-chain fee before performing a privileged action (release listing, unlock API quota, etc.). Agent-Devex generates:

1. A Soroban contract with **AgentGuard** (allowlist) and **AgentPay** (settlement).
2. An MCP server exposing `settle_and_execute`.

## Sample identities (testnet-shaped, not real keys)

| Role | Value |
|------|--------|
| Admin / source account | `GDDEMOADMINXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX` |
| Agent address | `GDDEMOAGENTXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX` |
| Contract id (after deploy) | `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA` |

Do **not** use these on a live network. They exist so slides and the dashboard can show the shape of IDs.

## Commands to run live

```bash
cargo run -- doctor
cargo run -- init pitch-demo --lang ts
cargo run -- validate --project-dir pitch-demo
# Requires stellar-cli + STELLAR_ACCOUNT
cargo run -- deploy --project-dir pitch-demo --network testnet
cargo run -- status --project-dir pitch-demo
```

Then copy `AGENTPAY_CONTRACT_ID` into `pitch-demo/.env` (from `.env.example`) and start `pitch-demo/agent`.
