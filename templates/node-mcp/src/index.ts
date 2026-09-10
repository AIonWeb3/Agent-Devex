import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { Keypair } from "@stellar/stellar-sdk";
import {
  buildDepositOperation,
  buildExecutePaymentOperation,
  mapStellarError,
  McpStellarError,
  requireEnv,
  simulateAndSubmit,
  stellarConnection,
} from "./stellar.js";

const server = new Server(
  { name: "{{PROJECT_NAME}}-mcp", version: "0.1.0" },
  { capabilities: { resources: {}, tools: {} } },
);

server.setRequestHandler(ListResourcesRequestSchema, async () => ({
  resources: [
    {
      uri: "stellar://wallet/state",
      name: "Wallet state",
      mimeType: "application/json",
      description: "Public key for the configured Stellar signer",
    },
  ],
}));

server.setRequestHandler(ReadResourceRequestSchema, async () => {
  const secret = requireEnv("STELLAR_SECRET_KEY");
  const keypair = Keypair.fromSecret(secret);
  return {
    contents: [
      {
        uri: "stellar://wallet/state",
        mimeType: "application/json",
        text: JSON.stringify({ address: keypair.publicKey() }, null, 2),
      },
    ],
  };
});

server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: "invoke_contract",
      description: "Simulate and submit deposit or execute_payment",
      inputSchema: {
        type: "object",
        properties: {
          method: { type: "string", enum: ["deposit", "execute_payment"] },
          payer: { type: "string" },
          agent: { type: "string" },
          amount: { type: "string" },
          escrow_id: { type: "string" },
          contract_id: { type: "string" },
        },
        required: ["method"],
      },
    },
  ],
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  try {
    const args = (request.params.arguments ?? {}) as Record<string, string>;
    const secret = requireEnv("STELLAR_SECRET_KEY");
    const contractId = args.contract_id ?? requireEnv("AGENTPAY_CONTRACT_ID");
    const { server: rpcServer, passphrase } = stellarConnection();
    const method = args.method;
    const op =
      method === "deposit"
        ? buildDepositOperation(contractId, args.payer, args.agent, args.amount)
        : buildExecutePaymentOperation(contractId, args.escrow_id, args.agent);
    const result = await simulateAndSubmit(rpcServer, passphrase, secret, op);
    return {
      content: [
        {
          type: "text",
          text: JSON.stringify({ ...result, contractId, method }, null, 2),
        },
      ],
    };
  } catch (err) {
    const mapped = err instanceof McpStellarError ? err : mapStellarError(err);
    return { content: [{ type: "text", text: mapped.message }], isError: true };
  }
});

const transport = new StdioServerTransport();
await server.connect(transport);
