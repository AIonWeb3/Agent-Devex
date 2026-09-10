"""{{PROJECT_NAME}} Python MCP server: balances + Soroban escrow calls."""

from __future__ import annotations

import asyncio
import json
import logging
import os
import sys
from typing import Any

from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import Resource, TextContent, Tool
from stellar_sdk import (
    Address,
    Keypair,
    Network,
    SorobanServer,
    TransactionBuilder,
    scval,
)
from stellar_sdk.exceptions import ConnectionError as StellarConnectionError
from stellar_sdk.exceptions import SdkError

LOG = logging.getLogger("{{PROJECT_NAME}}-mcp")


def configure_logging() -> None:
    handler = logging.StreamHandler(sys.stderr)
    handler.setFormatter(logging.Formatter("%(message)s"))
    LOG.setLevel(logging.INFO)
    LOG.handlers.clear()
    LOG.addHandler(handler)


def log_json(event: str, **fields: Any) -> None:
    LOG.info(json.dumps({"event": event, **fields}))


def require_env(name: str) -> str:
    value = os.environ.get(name)
    if not value:
        raise RuntimeError(f"missing required env {name}")
    return value


def stellar_server() -> SorobanServer:
    rpc_url = os.environ.get("STELLAR_RPC_URL", "https://soroban-testnet.stellar.org")
    return SorobanServer(rpc_url)


def network_passphrase() -> str:
    return os.environ.get(
        "STELLAR_NETWORK_PASSPHRASE", Network.TESTNET_NETWORK_PASSPHRASE
    )


def map_stellar_error(err: Exception) -> str:
    if isinstance(err, (TimeoutError, StellarConnectionError)):
        return f"stellar rpc timeout: {err}"
    if isinstance(err, SdkError):
        return f"stellar transaction failed: {err}"
    return str(err)


def build_deposit_tx(
    source: Any,
    contract_id: str,
    payer: str,
    agent: str,
    amount: int,
    passphrase: str,
) -> Any:
    return (
        TransactionBuilder(source, passphrase, base_fee=100)
        .append_invoke_contract_function_op(
            contract_id=contract_id,
            function_name="deposit",
            parameters=[
                Address(payer).to_xdr_sc_val(),
                Address(agent).to_xdr_sc_val(),
                scval.to_int128(amount),
            ],
        )
        .set_timeout(60)
        .build()
    )


def build_execute_payment_tx(
    source: Any,
    contract_id: str,
    escrow_id: int,
    agent: str,
    passphrase: str,
) -> Any:
    return (
        TransactionBuilder(source, passphrase, base_fee=100)
        .append_invoke_contract_function_op(
            contract_id=contract_id,
            function_name="execute_payment",
            parameters=[
                scval.to_uint64(escrow_id),
                Address(agent).to_xdr_sc_val(),
            ],
        )
        .set_timeout(60)
        .build()
    )


def sign_and_submit(server: SorobanServer, tx: Any, keypair: Keypair) -> dict[str, Any]:
    try:
        prepared = server.prepare_transaction(tx)
        prepared.sign(keypair)
        send = server.send_transaction(prepared)
        return {"hash": send.hash, "status": send.status}
    except Exception as err:  # noqa: BLE001
        raise RuntimeError(map_stellar_error(err)) from err


def create_server() -> Server:
    server = Server("{{PROJECT_NAME}}-mcp")

    @server.list_resources()
    async def list_resources() -> list[Resource]:
        return [
            Resource(
                uri="stellar://account/balance",
                name="Stellar account balance",
                mimeType="application/json",
                description="Native balance for the configured signer account",
            )
        ]

    @server.read_resource()
    async def read_resource(uri: str) -> str:
        log_json("read_resource", uri=str(uri))
        secret = require_env("STELLAR_SECRET_KEY")
        keypair = Keypair.from_secret(secret)
        rpc = stellar_server()
        account = rpc.load_account(keypair.public_key)
        return json.dumps(
            {"address": keypair.public_key, "sequence": account.sequence},
            indent=2,
        )

    @server.list_tools()
    async def list_tools() -> list[Tool]:
        return [
            Tool(
                name="invoke_contract",
                description="Build, sign, and submit deposit or execute_payment",
                inputSchema={
                    "type": "object",
                    "properties": {
                        "method": {
                            "type": "string",
                            "enum": ["deposit", "execute_payment"],
                        },
                        "payer": {"type": "string"},
                        "agent": {"type": "string"},
                        "amount": {"type": "string"},
                        "escrow_id": {"type": "string"},
                        "contract_id": {"type": "string"},
                    },
                    "required": ["method"],
                },
            )
        ]

    @server.call_tool()
    async def call_tool(name: str, arguments: dict[str, Any]) -> list[TextContent]:
        log_json("call_tool", name=name, arguments=arguments)
        if name != "invoke_contract":
            raise RuntimeError(f"unknown tool {name}")
        secret = require_env("STELLAR_SECRET_KEY")
        cid = arguments.get("contract_id") or require_env("AGENTPAY_CONTRACT_ID")
        keypair = Keypair.from_secret(secret)
        rpc = stellar_server()
        source = rpc.load_account(keypair.public_key)
        passphrase = network_passphrase()
        method = arguments["method"]
        if method == "deposit":
            tx = build_deposit_tx(
                source,
                cid,
                arguments["payer"],
                arguments["agent"],
                int(arguments["amount"]),
                passphrase,
            )
        elif method == "execute_payment":
            tx = build_execute_payment_tx(
                source,
                cid,
                int(arguments["escrow_id"]),
                arguments["agent"],
                passphrase,
            )
        else:
            raise RuntimeError(f"unsupported method {method}")
        result = sign_and_submit(rpc, tx, keypair)
        result["contractId"] = cid
        result["method"] = method
        return [TextContent(type="text", text=json.dumps(result, indent=2))]

    return server


async def run() -> None:
    configure_logging()
    log_json("server_start", backend="python-mcp")
    server = create_server()
    async with stdio_server() as (read, write):
        await server.run(read, write, server.create_initialization_options())


def main() -> None:
    asyncio.run(run())


if __name__ == "__main__":
    main()
