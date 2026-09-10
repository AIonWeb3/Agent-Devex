use anyhow::Result;
use std::path::Path;

/// Initialize a local MCP server.
///
/// This does not bind a socket or start an MCP process; it is a stub so the
/// CLI surface and config loading can land first.
pub fn cmd_run(project_dir: &Path, port: Option<u16>) -> Result<()> {
    let _ = (project_dir, port);
    tracing::info!("initializing local MCP server (stub; not listening)");
    Ok(())
}
