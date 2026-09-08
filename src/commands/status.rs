use anyhow::Result;
use std::path::Path;

use crate::config;
use crate::errors::AgentDevexError;
use crate::output;
use crate::paths;

/// Print local project + contract configuration used by the MCP server.
pub fn cmd_status(project_dir: &Path) -> Result<()> {
    if !paths::contract_manifest(project_dir).is_file() {
        return Err(AgentDevexError::ConfigNotFound {
            path: paths::contract_manifest(project_dir),
        }
        .into());
    }

    let cfg = config::load_or_default(project_dir)?;
    let state = crate::state::load_optional(project_dir)?.unwrap_or_default();
    let network = state
        .network
        .as_deref()
        .or(cfg.network.as_deref())
        .unwrap_or("testnet");
    let lang = cfg
        .default_lang
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    let contract_id = std::env::var("AGENTPAY_CONTRACT_ID")
        .ok()
        .or(state.contract_id.clone());
    let rpc = std::env::var("STELLAR_RPC_URL")
        .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string());

    output::success(format!("Project {}", project_dir.display()));
    output::hint(format!("  network:     {network}"));
    output::hint(format!("  mcp lang:    {lang}"));
    output::hint(format!("  rpc:         {rpc}"));
    match contract_id {
        Some(id) => output::hint(format!("  contract id: {id}")),
        None => output::warn("  contract id: (not set) export AGENTPAY_CONTRACT_ID after deploy"),
    }
    Ok(())
}
