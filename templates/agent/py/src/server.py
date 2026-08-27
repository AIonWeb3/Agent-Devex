"""MCP stdio server: LLM tool calls → signed Soroban invoke on Stellar.

The host LLM calls `settle_and_execute`. This process maps structured arguments onto
`execute_agent_action`, builds a Soroban transaction, signs with STELLAR_SECRET_KEY, and
submits to STELLAR_RPC_URL. The prompt is recorded in the tool result, not executed.
"""

from __future__ import annotations

import json
import os

from mcp.server.fastmcp import FastMCP
from stellar_sdk import Address, Keypair, Network, SorobanServer, TransactionBuilder, scval

mcp = FastMCP("agent-devex-stellar")


def _require_env(name: str) -> str:
    value = os.environ.get(name)
    if not value:
        raise RuntimeError(f"missing required env {name}")
    return value


@mcp.tool()
def settle_and_execute(
    prompt: str,
    agent_address: str,
    action_id: str,
    amount: str,
    contract_id: str | None = None,
) -> str:
    """Verify the agent, settle AgentPay, then invoke execute_agent_action on Soroban."""
    secret = _require_env("STELLAR_SECRET_KEY")
    rpc_url = os.environ.get("STELLAR_RPC_URL", "https://soroban-testnet.stellar.org")
    passphrase = os.environ.get(
        "STELLAR_NETWORK_PASSPHRASE", Network.TESTNET_NETWORK_PASSPHRASE
    )
    cid = contract_id or _require_env("AGENTPAY_CONTRACT_ID")

    keypair = Keypair.from_secret(secret)
    server = SorobanServer(rpc_url)
    source = server.load_account(keypair.public_key)

    tx = (
        TransactionBuilder(source, passphrase, base_fee=100)
        .append_invoke_contract_function_op(
            contract_id=cid,
            function_name="execute_agent_action",
            parameters=[
                Address(agent_address).to_xdr_sc_val(),
                scval.to_symbol(action_id),
                scval.to_int128(int(amount)),
            ],
        )
        .set_timeout(60)
        .build()
    )
    prepared = server.prepare_transaction(tx)
    prepared.sign(keypair)
    send = server.send_transaction(prepared)
    return json.dumps(
        {
            "prompt": prompt,
            "hash": send.hash,
            "status": send.status,
            "contractId": cid,
        },
        indent=2,
    )


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
