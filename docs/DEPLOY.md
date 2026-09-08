# Deployment

## CLI binary

```bash
cargo build --release
# artifact: target/release/agent-devex(.exe)
```

Install locally:

```bash
cargo install --path .
```

## Generated Soroban contract

From a project created by `agent-devex init`:

1. Install [stellar-cli](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli).
2. `rustup target add wasm32-unknown-unknown`
3. Set `STELLAR_ACCOUNT` to a funded testnet identity.
4. `agent-devex deploy --project-dir . --network testnet`

Mainnet: pass `--network mainnet` only after security review. This MVP is **testnet-first**.

## MCP server

Do not deploy `STELLAR_SECRET_KEY` in git. Use host env / secret manager. The generated `.gitignore` excludes `.env`.

## Pitch dashboard

The `dashboard/` folder is static. Host on GitHub Pages, Netlify, or open `index.html` locally. No backend.
