//! Agent-Devex CLI: scaffold a Soroban + MCP monorepo and stub-deploy to Stellar testnet.
//!
//! File generation: templates live on disk under `templates/` and are compiled into the
//! binary with [`include_str!`]. That keeps large MCP/Soroban sources editable as normal
//! files instead of giant string literals in Rust. At `init` time we write those bytes
//! (with `{{PROJECT_NAME}}` substitution) via [`crate::scaffold`].

mod scaffold;

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "agent-devex", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a monorepo: `contracts/` (Soroban + AgentPay) and `agent/` (MCP server).
    Init {
        project_name: String,
        /// MCP server language
        #[arg(long, value_enum)]
        lang: Lang,
    },
    /// Compile the Soroban contract and deploy it to a Stellar network (testnet by default).
    Deploy {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
        #[arg(long, default_value = "testnet")]
        network: String,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Lang {
    Ts,
    Py,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Init { project_name, lang } => cmd_init(&project_name, lang),
        Commands::Deploy {
            project_dir,
            network,
        } => cmd_deploy(&project_dir, &network),
    }
}

fn cmd_init(project_name: &str, lang: Lang) -> Result<()> {
    let root = PathBuf::from(project_name);
    if root.exists() {
        let empty = root
            .read_dir()
            .with_context(|| format!("cannot read {}", root.display()))?
            .next()
            .is_none();
        if !empty {
            bail!(
                "directory {} already exists and is not empty",
                root.display()
            );
        }
    }

    scaffold::write_project(&root, project_name, lang)?;

    eprintln!("Created {project_name}/");
    eprintln!("  contracts/agent_pay_integration  Soroban + AgentPay/AgentGuard");
    match lang {
        Lang::Ts => {
            eprintln!("  agent/                          TypeScript MCP server");
            eprintln!("Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml");
            eprintln!("      cd agent && npm install && npx tsx src/index.ts");
        }
        Lang::Py => {
            eprintln!("  agent/                          Python MCP server");
            eprintln!("Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml");
            eprintln!("      cd agent && uv sync && uv run python src/server.py");
        }
    }
    Ok(())
}

fn cmd_deploy(project_dir: &Path, network: &str) -> Result<()> {
    let contract_dir = project_dir.join("contracts").join("agent_pay_integration");
    if !contract_dir.join("Cargo.toml").is_file() {
        bail!(
            "missing {} — run `agent-devex init` first or pass --project-dir",
            contract_dir.join("Cargo.toml").display()
        );
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
            )
        }
    }
}

fn run_stellar(args: &[&str], cwd: &Path, label: &str) -> Result<()> {
    let status = Command::new("stellar")
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| {
            format!("{label} failed to start — is stellar-cli installed and on PATH?")
        })?;
    if !status.success() {
        bail!("{label} exited with {status}");
    }
    Ok(())
}

fn find_wasm(contract_dir: &Path) -> Result<PathBuf> {
    let target = contract_dir.join("target").join("wasm32-unknown-unknown");
    let mut found = Vec::new();
    for profile in ["release", "debug"] {
        let dir = target.join(profile);
        if dir.is_dir() {
            for entry in std::fs::read_dir(&dir).with_context(|| dir.display().to_string())? {
                let path = entry?.path();
                if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
                    found.push(path);
                }
            }
        }
    }
    found
        .into_iter()
        .next()
        .context("no .wasm after build — check stellar contract build output")
}
