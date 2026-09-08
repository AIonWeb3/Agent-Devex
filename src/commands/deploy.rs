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
            process::run_stellar(
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
            crate::next_steps::after_deploy_success(&network);
            Ok(())
        }
    }
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
    found
        .into_iter()
        .next()
        .ok_or(AgentDevexError::WasmNotFound)
}
