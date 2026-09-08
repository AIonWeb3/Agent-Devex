//! Library root for Agent-Devex CLI.
//!
//! Provides the core implementation for the CLI commands, configuration management,
//! error handling, and project scaffolding templates.

pub mod commands;
pub mod config;
pub mod errors;
pub mod fsutil;
pub mod names;
pub mod next_steps;
pub mod panic;
pub mod output;
pub mod paths;
pub mod process;
pub mod scaffold;
pub mod secrets;
pub mod state;

pub use errors::{AgentDevexError, Result};

/// Supported languages for the generated MCP server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Lang {
    /// TypeScript MCP server (runs via Node.js / npx)
    Ts,
    /// Python MCP server (runs via Python / uv)
    Py,
}

impl Lang {
    pub fn as_config_str(self) -> &'static str {
        match self {
            Lang::Ts => "ts",
            Lang::Py => "py",
        }
    }
}
