//! Canonical paths inside an Agent-Devex generated project.
//!
//! Command implementations should resolve locations through this module so
//! `init`, `deploy`, and later commands stay aligned with the scaffold layout.

use std::path::{Path, PathBuf};

/// Directory name for generated Soroban contracts.
pub const CONTRACTS_DIR: &str = "contracts";

/// Crate name of the default AgentPay integration contract.
pub const AGENT_PAY_CONTRACT: &str = "agent_pay_integration";

/// Directory name for the generated MCP server.
pub const AGENT_DIR: &str = "agent";

/// Root of the default contract crate: `<project>/contracts/agent_pay_integration`.
pub fn contract_crate_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(CONTRACTS_DIR).join(AGENT_PAY_CONTRACT)
}

/// Manifest path for the default contract crate.
pub fn contract_manifest(project_dir: &Path) -> PathBuf {
    contract_crate_dir(project_dir).join("Cargo.toml")
}

/// MCP agent directory: `<project>/agent`.
pub fn agent_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(AGENT_DIR)
}

/// Stellar WASM output root for a contract crate (`target/wasm32-unknown-unknown`).
pub fn wasm_target_root(contract_dir: &Path) -> PathBuf {
    contract_dir.join("target").join("wasm32-unknown-unknown")
}
