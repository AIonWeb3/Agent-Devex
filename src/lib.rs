//! Library root for Agent-Devex CLI.
//!
//! Provides the core implementation for the CLI commands, configuration management,
//! error handling, and project scaffolding templates.

pub mod commands;
pub mod config;
pub mod errors;
pub mod paths;
pub mod process;
pub mod scaffold;

/// Supported languages for the generated MCP server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Lang {
    /// TypeScript MCP server (runs via Node.js / npx)
    Ts,
    /// Python MCP server (runs via Python / uv)
    Py,
}
