//! Agent-Devex CLI: scaffold a Soroban + MCP monorepo and stub-deploy to Stellar testnet.
//!
//! File generation: templates live on disk under `templates/` and are compiled into the
//! binary with [`include_str!`]. That keeps large MCP/Soroban sources editable as normal
//! files instead of giant string literals in Rust. At `init` time we write those bytes
//! (with `{{PROJECT_NAME}}` substitution) via [`agent_devex::scaffold`].

use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use agent_devex::Lang;
use agent_devex::commands::{
    deploy::cmd_deploy, doctor::cmd_doctor, init::cmd_init, validate::cmd_validate,
};
use agent_devex::config;

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
        /// MCP server language (prompted on a TTY when omitted)
        #[arg(long, value_enum)]
        lang: Option<Lang>,
    },
    /// Compile the Soroban contract and deploy it to a Stellar network (testnet by default).
    Deploy {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
        #[arg(long)]
        network: Option<String>,
    },
    /// Check that a generated project has the expected contract and MCP layout.
    Validate {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
    },
    /// Diagnose required and optional tools on this machine.
    Doctor,
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
        } => cmd_deploy(&project_dir, network.as_deref()),
        Commands::Validate { project_dir } => cmd_validate(&project_dir),
        Commands::Doctor => cmd_doctor(),
    }
}
