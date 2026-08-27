# Agent-Devex

CLI to scaffold, test, and deploy AI-to-Web3 integrations on Stellar: a Soroban contract
pre-wired with AgentPay/AgentGuard interfaces and an MCP server stub (`settle_and_execute`).

```bash
cargo run -- init my-agent --lang ts
cargo run -- init my-agent --lang py
cargo run -- deploy --project-dir my-agent --network testnet
```

Requires [stellar-cli](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) on `PATH` for `deploy`. Set `STELLAR_ACCOUNT` to run deploy instead of printing the command.
