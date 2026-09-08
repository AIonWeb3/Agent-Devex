use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Result;

use crate::errors::AgentDevexError;

pub fn cmd_deploy(project_dir: &Path, network: &str) -> Result<()> {
    let contract_dir = project_dir.join("contracts").join("agent_pay_integration");
    if !contract_dir.join("Cargo.toml").is_file() {
        return Err(AgentDevexError::ConfigNotFound {
            path: contract_dir.join("Cargo.toml"),
        }
        .into());
    }

    run_stellar(
        &["contract", "build"],
        &contract_dir,
        "stellar contract build",
    )?;

    let wasm = find_wasm(&contract_dir)?;
    let source = std::env::var("STELLAR_ACCOUNT").ok();
    match source {
        None => {
            eprintln!(
                "Built {}. Set STELLAR_ACCOUNT and re-run deploy, or run:",
                wasm.display()
            );
            eprintln!(
                "  stellar contract deploy --network {network} --source-account <ACCOUNT> --wasm {}",
                wasm.display()
            );
            Ok(())
        }
        Some(account) => {
            let wasm_s = wasm.to_string_lossy();
            run_stellar(
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

fn run_stellar(args: &[&str], cwd: &Path, label: &str) -> Result<(), AgentDevexError> {
    let status = Command::new("stellar")
        .args(args)
        .current_dir(cwd)
        .status()
        .map_err(|source| AgentDevexError::StellarSpawn {
            label: label.to_string(),
            source,
        })?;
    if !status.success() {
        return Err(AgentDevexError::StellarFailed {
            label: label.to_string(),
            status,
        });
    }
    Ok(())
}

fn find_wasm(contract_dir: &Path) -> Result<PathBuf, AgentDevexError> {
    let target = contract_dir.join("target").join("wasm32-unknown-unknown");
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
