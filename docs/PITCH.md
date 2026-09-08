# Client pitch — Agent-Devex

## Problem

AI products need to take **paid, authorized actions on a blockchain**. Teams currently assemble Soroban contracts, signing, RPC, and MCP servers by hand. That is slow, error-prone, and hard to demo.

## Solution

A CLI that generates a production-shaped monorepo: AgentGuard allowlist + AgentPay settlement on Soroban, plus TypeScript or Python MCP with `settle_and_execute`.

## Target users

- Web3 startups adding LLM agents
- Integrators building “agent pays then acts” flows on Stellar
- Platform teams who want a repeatable DX, not a one-off gist

## Value

- Time-to-first-tool-call measured in minutes
- Secrets stay in `.env`, not templates
- `doctor` / `validate` / `status` make live demos recoverable

## What is demo-ready

- `init`, `deploy` (stellar-cli), `validate`, `doctor`, `status`
- Static pitch dashboard
- Tests + CI

## What is not production-mainnet

- Local AgentPay/AgentGuard interfaces (not a published protocol crate)
- No real money path without your own keys, audits, and mainnet policy
