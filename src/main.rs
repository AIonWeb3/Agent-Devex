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
    compile::cmd_compile, deploy::cmd_deploy, doctor::cmd_doctor, init::cmd_init, run::cmd_run,
    status::cmd_status, validate::cmd_validate,
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
        /// Directory name for the generated project (`agent.toml` is written here).
        #[arg(value_name = "PROJECT_NAME")]
        project_name: String,
        /// MCP server language (prompted on a TTY when omitted)
        #[arg(long, value_enum)]
        lang: Option<Lang>,
    },
    /// Load project config and run the Soroban compile workflow (currently a stub).
    Compile {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
    },
    /// Initialize a local MCP server (stub; does not bind a socket).
    Run {
        /// Listen port (defaults to agent.toml / 3000).
        #[arg(long)]
        port: Option<u16>,
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
    },
    /// Compile the Soroban contract and deploy it to a Stellar network (testnet by default).
    Deploy {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
        /// Target network (`testnet` or `mainnet`).
        #[arg(long, value_name = "NETWORK")]
        network: Option<String>,
    },
    /// Check that a generated project has the expected contract and MCP layout.
    Validate {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
    },
    /// Diagnose required and optional tools on this machine.
    Doctor,
    /// Show local network, language, and contract id configuration.
    Status {
        #[arg(long, default_value = ".")]
        project_dir: PathBuf,
        /// Exit with an error when no contract id is configured
        #[arg(long)]
        require_id: bool,
    },
}

fn load_dotenv_files(cli: &Cli) {
    // Process environment always wins. `.env` only fills missing keys (never logged).
    let _ = dotenvy::dotenv();
    let extra = match &cli.command {
        Commands::Deploy { project_dir, .. }
        | Commands::Validate { project_dir }
        | Commands::Status { project_dir, .. }
        | Commands::Compile { project_dir }
        | Commands::Run { project_dir, .. } => Some(project_dir.as_path()),
        Commands::Init { .. } | Commands::Doctor => None,
    };
    if let Some(dir) = extra {
        let path = dir.join(".env");
        if path.is_file() {
            let _ = dotenvy::from_path(&path);
        }
    }
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

fn init_services() {
    agent_devex::panic::install_panic_handler();
    init_tracing();
    tracing::debug!("tracing initialized");
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "agent-devex runtime services ready"
    );
}

fn main() {
    init_services();
    if let Err(err) = run() {
        tracing::error!("{err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    load_dotenv_files(&cli);
    if let Some(cfg) = config::load_optional(Path::new("."))?
        && let Ok(encoded) = toml::to_string(&cfg)
    {
        tracing::debug!(encoded, "loaded {}", config::CONFIG_FILE_NAME);
    }

    match cli.command {
        Commands::Init { project_name, lang } => cmd_init(&project_name, lang),
        Commands::Compile { project_dir } => cmd_compile(&project_dir),
        Commands::Run { project_dir, port } => cmd_run(&project_dir, port),
        Commands::Deploy {
            project_dir,
            network,
        } => cmd_deploy(&project_dir, network.as_deref()),
        Commands::Validate { project_dir } => cmd_validate(&project_dir),
        Commands::Doctor => cmd_doctor(),
        Commands::Status {
            project_dir,
            require_id,
        } => cmd_status(&project_dir, require_id),
    }
}
