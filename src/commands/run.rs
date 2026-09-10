use anyhow::Result;
use std::path::Path;

use crate::config::{AgentConfig, DEFAULT_MCP_PORT, McpConfig};

/// CLI `--port` wins, then `agent.toml`, then [`DEFAULT_MCP_PORT`].
pub fn resolve_mcp_port(project_dir: &Path, cli_port: Option<u16>) -> crate::errors::Result<u16> {
    let port = match cli_port {
        Some(port) => port,
        None => AgentConfig::load(project_dir)
            .map(|cfg| cfg.mcp.port)
            .unwrap_or(DEFAULT_MCP_PORT),
    };
    McpConfig::validate_port(port)
}

/// Initialize a local MCP server.
///
/// This does not bind a socket or start an MCP process; it is a stub so the
/// CLI surface and config loading can land first.
pub fn cmd_run(project_dir: &Path, port: Option<u16>) -> Result<()> {
    let port = resolve_mcp_port(project_dir, port)?;
    tracing::info!(port, "initializing local MCP server (stub; not listening)");
    Ok(())
}
