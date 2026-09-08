use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::errors::AgentDevexError;
use crate::output;
use crate::paths;
use crate::process;

pub fn cmd_deploy(project_dir: &Path, network: &str) -> Result<()> {
    let contract_dir = paths::contract_crate_dir(project_dir);
    let manifest = paths::contract_manifest(project_dir);
    if !manifest.is_file() {
        return Err(AgentDevexError::ConfigNotFound { path: manifest }.into());
    }

    process::run_stellar(
        &["contract", "build"],
        &contract_dir,
        "stellar contract build",
    )?;

    let wasm = find_wasm(&contract_dir)?;
    let source = std::env::var("STELLAR_ACCOUNT").ok();
    match source {
        None => {
            output::warn(format!(
                "Built {}. Set STELLAR_ACCOUNT and re-run deploy, or run:",
                wasm.display()
            ));
            output::hint(format!(
                "  stellar contract deploy --network {network} --source-account <ACCOUNT> --wasm {}",
                wasm.display()
            ));
            Ok(())
        }
        Some(account) => {
            let wasm_s = wasm.to_string_lossy();
            process::run_stellar(
                &[
                    "contract",
                    "deploy",
                    "--network",
                    network,
                    "--source-account",
                    &account,
                    "--wasm",
                    wasm_s.as_ref(),
                ],
                project_dir,
                "stellar contract deploy",
            )?;
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
