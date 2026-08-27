//! Agent-Devex CLI: scaffold a Soroban + MCP monorepo and stub-deploy to Stellar testnet.
//!
//! File generation: templates live on disk under `templates/` and are compiled into the
//! binary with [`include_str!`]. That keeps large MCP/Soroban sources editable as normal
//! files instead of giant string literals in Rust. At `init` time we write those bytes
//! (with `{{PROJECT_NAME}}` substitution) via [`crate::scaffold`].

mod config;
mod errors;
mod scaffold;

use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::errors::AgentDevexError;

#[derive(Parser)]
#[command(
    name = "agent-devex",
    version,
    author,
    about,
    long_about = None
)]
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

fn ansi_logs_enabled() -> bool {
    // Honor https://no-color.org — skip ANSI when the user asks for plain output.
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    std::io::stdout().is_terminal()
}

fn init_tracing() {
    // tracing-subscriber's fmt layer maps levels to ANSI colors when `ansi` is on:
    // ERROR red, WARN yellow, INFO green, DEBUG blue, TRACE purple.
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .with_ansi(ansi_logs_enabled())
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new(tracing::Level::INFO.as_str())
            }),
        )
        .init();
}

fn main() -> Result<()> {
    // Release builds: friendly dump instead of a raw backtrace. Debug / RUST_BACKTRACE=1 keep the default hook.
    human_panic::setup_panic!(
        human_panic::metadata!()
            .authors("AIonWeb3")
            .homepage("https://github.com/AIonWeb3/Agent-Devex")
            .support("- Open an issue: https://github.com/AIonWeb3/Agent-Devex/issues")
    );
    init_tracing();
    tracing::debug!("tracing initialized");
    if let Some(cfg) = config::load_optional(Path::new("."))?
        && let Ok(encoded) = toml::to_string(&cfg)
    {
        tracing::debug!(encoded, "loaded {}", config::CONFIG_FILE_NAME);
    }

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
            .map_err(|source| AgentDevexError::IoError {
                path: root.clone(),
                source,
            })?
            .next()
            .is_none();
        if !empty {
            return Err(AgentDevexError::DirectoryNotEmpty { path: root }.into());
        }
    }

    scaffold::write_project(&root, project_name, lang)?;

    eprintln!("Created {project_name}/");
    eprintln!("  contracts/agent_pay_integration  Soroban + AgentPay/AgentGuard");
    match lang {
        Lang::Ts => {
            eprintln!("  agent/                          TypeScript MCP server");
            eprintln!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            );
            eprintln!("      cd agent && npm install && npx tsx src/index.ts");
        }
        Lang::Py => {
            eprintln!("  agent/                          Python MCP server");
            eprintln!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            );
            eprintln!("      cd agent && uv sync && uv run python src/server.py");
        }
    }
    Ok(())
}

fn cmd_deploy(project_dir: &Path, network: &str) -> Result<()> {
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
