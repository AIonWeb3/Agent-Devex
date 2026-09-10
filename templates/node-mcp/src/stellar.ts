import {
  Address,
  BASE_FEE,
  Contract,
  Keypair,
  nativeToScVal,
  rpc,
  TransactionBuilder,
} from "@stellar/stellar-sdk";

export class McpStellarError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "McpStellarError";
  }
}

export function mapStellarError(err: unknown): McpStellarError {
  const message = err instanceof Error ? err.message : String(err);
  if (/timeout|ETIMEDOUT|ECONNRESET/i.test(message)) {
    return new McpStellarError(`stellar rpc timeout: ${message}`);
  }
  return new McpStellarError(`stellar transaction failed: ${message}`);
}

export function requireEnv(name: string): string {
  const v = process.env[name];
  if (!v) {
    throw new McpStellarError(`missing required env ${name}`);
  }
  return v;
}

export function stellarConnection(): { server: rpc.Server; passphrase: string } {
  const rpcUrl = process.env.STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org";
  const passphrase =
    process.env.STELLAR_NETWORK_PASSPHRASE ?? "Test SDF Network ; September 2015";
  return { server: new rpc.Server(rpcUrl), passphrase };
}

export function buildDepositOperation(
  contractId: string,
  payer: string,
  agent: string,
  amount: string,
) {
  const contract = new Contract(contractId);
  return contract.call(
    "deposit",
    Address.fromString(payer).toScVal(),
    Address.fromString(agent).toScVal(),
    nativeToScVal(amount, { type: "i128" }),
  );
}

export function buildExecutePaymentOperation(
  contractId: string,
  escrowId: string,
  agent: string,
) {
  const contract = new Contract(contractId);
  return contract.call(
    "execute_payment",
    nativeToScVal(escrowId, { type: "u64" }),
    Address.fromString(agent).toScVal(),
  );
}

export async function simulateAndSubmit(
  server: rpc.Server,
  passphrase: string,
  secret: string,
  operation: ReturnType<Contract["call"]>,
): Promise<{ hash: string; status: string }> {
  try {
    const keypair = Keypair.fromSecret(secret);
    const account = await server.getAccount(keypair.publicKey());
    const built = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: passphrase,
    })
      .addOperation(operation)
      .setTimeout(60)
      .build();
    const prepared = await server.prepareTransaction(built);
    prepared.sign(keypair);
    const send = await server.sendTransaction(prepared);
    return { hash: send.hash, status: send.status };
  } catch (err) {
    throw mapStellarError(err);
  }
}
