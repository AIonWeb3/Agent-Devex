/**
 * MCP stdio server: LLM tool calls → signed Soroban invoke on Stellar.
 *
 * Flow: the host LLM lists tools, then calls `settle_and_execute` with a prompt plus
 * chain args. This process does not interpret the prompt as code; it maps structured
 * arguments onto `execute_agent_action`, builds a Soroban transaction, signs it with
 * STELLAR_SECRET_KEY, and submits it to STELLAR_RPC_URL.
 */

import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  Address,
  BASE_FEE,
  Contract,
  Keypair,
  nativeToScVal,
  rpc,
  TransactionBuilder,
} from "@stellar/stellar-sdk";
import { z } from "zod";

const settleSchema = {
  prompt: z
    .string()
    .describe("Natural-language intent from the LLM (logged; not executed as code)"),
  agent_address: z.string().describe("Stellar/Soroban address of the agent"),
  action_id: z.string().describe("Symbol-like action id passed to the contract"),
  amount: z.string().describe("i128 payment amount as a decimal string"),
  contract_id: z
    .string()
    .optional()
    .describe("Override AGENTPAY_CONTRACT_ID"),
};

async function settleAndExecute(args: {
  prompt: string;
  agent_address: string;
  action_id: string;
  amount: string;
  contract_id?: string;
}): Promise<string> {
  const secret = requireEnv("STELLAR_SECRET_KEY");
  const rpcUrl = process.env.STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org";
  const passphrase =
    process.env.STELLAR_NETWORK_PASSPHRASE ?? "Test SDF Network ; September 2015";
  const contractId = args.contract_id ?? requireEnv("AGENTPAY_CONTRACT_ID");

  const keypair = Keypair.fromSecret(secret);
  const server = new rpc.Server(rpcUrl);
  const account = await server.getAccount(keypair.publicKey());
  const contract = new Contract(contractId);

  const built = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: passphrase,
  })
    .addOperation(
      contract.call(
        "execute_agent_action",
        Address.fromString(args.agent_address).toScVal(),
        nativeToScVal(args.action_id, { type: "symbol" }),
        nativeToScVal(args.amount, { type: "i128" }),
      ),
    )
    .setTimeout(60)
    .build();

  const prepared = await server.prepareTransaction(built);
  prepared.sign(keypair);
  const send = await server.sendTransaction(prepared);

  return JSON.stringify(
    {
      prompt: args.prompt,
      hash: send.hash,
      status: send.status,
      contractId,
    },
    null,
    2,
  );
}

function requireEnv(name: string): string {
  const v = process.env[name];
  if (!v) {
    throw new Error(`missing required env ${name}`);
  }
  return v;
}

const server = new McpServer({
  name: "agent-devex-stellar",
  version: "0.1.0",
});

server.tool(
  "settle_and_execute",
  "Verify the agent, settle AgentPay, then invoke execute_agent_action on Soroban.",
  settleSchema,
  async (args) => {
    try {
      const text = await settleAndExecute(args);
      return { content: [{ type: "text", text }] };
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      return { content: [{ type: "text", text: message }], isError: true };
    }
  },
);

const transport = new StdioServerTransport();
await server.connect(transport);
