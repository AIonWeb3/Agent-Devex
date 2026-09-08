# Live demo script (client pitch)

Run from the Agent-Devex repo root.

## 0. Open the visual story

Open `dashboard/index.html` in a browser. Walk problem → workflow → value → empty state → form.

## 1. Toolchain

```bash
cargo run -- doctor
```

## 2. Scaffold

```bash
cargo run -- init pitch-demo --lang ts
cargo run -- validate --project-dir pitch-demo
```

## 3. Deploy (optional live testnet)

Requires stellar-cli and `STELLAR_ACCOUNT`.

```bash
export STELLAR_ACCOUNT=your-alias
cargo run -- deploy --project-dir pitch-demo --network testnet
cargo run -- status --project-dir pitch-demo --require-id
```

If you cannot deploy, copy `examples/demo-agent/state.toml` into `pitch-demo/.agent-devex/state.toml` for a **shape-only** status demo (not a real contract).

## 4. MCP

```bash
cd pitch-demo/agent
cp ../.env.example .env   # fill real secrets locally
npm install
npx tsx src/index.ts
```

Point Cursor MCP at this stdio process and call `settle_and_execute`.
