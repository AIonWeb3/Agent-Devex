use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::errors::AgentDevexError;
use crate::paths;
use crate::process;

pub fn cmd_deploy(project_dir: &Path, network: Option<&str>) -> Result<()> {
    let cfg = crate::config::load_or_default(project_dir)?;
    let network = network
        .map(str::to_string)
        .or(cfg.network)
        .unwrap_or_else(|| "testnet".to_string());
    validate_network_name(&network)?;
    let contract_dir = paths::contract_crate_dir(project_dir);
    let manifest = paths::contract_manifest(project_dir);
    if !manifest.is_file() {
        return Err(AgentDevexError::ConfigNotFound { path: manifest }.into());
    }

    tracing::info!(project = %project_dir.display(), network = %network, "building Soroban contract");
    process::run_stellar(
        &["contract", "build"],
        &contract_dir,
        "stellar contract build",
    )?;
    tracing::info!("contract build finished");

    let wasm = find_wasm(&contract_dir)?;
    let source = std::env::var("STELLAR_ACCOUNT").ok();
    match source {
        None => {
            crate::next_steps::after_deploy_without_account(&wasm, &network);
            Ok(())
        }
        Some(account) => {
            let wasm_s = wasm.to_string_lossy();
            tracing::info!(account, "deploying wasm to network");
            let out = process::run_stellar_output(
                &[
                    "contract",
                    "deploy",
                    "--network",
                    network.as_str(),
                    "--source-account",
                    &account,
                    "--wasm",
                    wasm_s.as_ref(),
                ],
                project_dir,
                "stellar contract deploy",
            )?;
            if !out.trim().is_empty() {
                crate::output::hint(out.trim());
            }
            if let Some(id) = process::parse_contract_id(&out) {
                crate::output::success(format!("Contract id {id}"));
                let mut state = crate::state::load_optional(project_dir)?.unwrap_or_default();
                state.network = Some(network.clone());
                state.contract_id = Some(id);
                state.wasm_path = Some(wasm.display().to_string());
                state.last_deployed_unix = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs());
                crate::state::save(project_dir, &state)?;
            }
            crate::next_steps::after_deploy_success(&network);
            Ok(())
        }
    }
}

fn validate_network_name(network: &str) -> Result<(), AgentDevexError> {
    if network.is_empty()
        || !network
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(AgentDevexError::InvalidConfigValue {
            key: "network".to_string(),
            value: network.to_string(),
        });
    }
    Ok(())
}

fn find_wasm(contract_dir: &Path) -> Result<PathBuf, AgentDevexError> {
    let target = paths::wasm_target_root(contract_dir);
    let mut found = Vec::new();
    for profile in ["release", "debug"] {
        let dir = target.join(profile);
        if dir.is_dir() {
            for entry in std::fs::read_dir(&dir).map_err(|source| AgentDevexError::IoError {
                path: dir.clone(),
                source,
            })? {
                let path = entry
                    .map_err(|source| AgentDevexError::IoError {
                        path: dir.clone(),
                        source,
                    })?
                    .path();
                if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
                    found.push(path);
                }
            }
        }
    }
    found.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    found.pop().ok_or(AgentDevexError::WasmNotFound)
}
