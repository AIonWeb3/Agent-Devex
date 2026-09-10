use anyhow::Result;
use std::path::Path;

use crate::config::AgentConfig;

/// Load `agent.toml` as a prerequisite for compilation.
///
/// Soroban `stellar contract build` is not invoked here; this is the stub
/// architecture for a later compile implementation.
pub fn cmd_compile(project_dir: &Path) -> Result<()> {
    let _cfg = AgentConfig::load(project_dir)?;
    Ok(())
}
