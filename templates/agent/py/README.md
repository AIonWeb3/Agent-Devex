# {{PROJECT_NAME}} MCP agent (Python)

Bridges LLM tool calls to the Soroban `execute_agent_action` entrypoint.

```bash
uv sync
export STELLAR_SECRET_KEY=S...
export AGENTPAY_CONTRACT_ID=C...
export STELLAR_RPC_URL=https://soroban-testnet.stellar.org
export STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
uv run python src/server.py
```
