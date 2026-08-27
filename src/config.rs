//! Optional `agent-devex.toml` in the working directory (`serde` + `toml`).

use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::errors::AgentDevexError;

pub const CONFIG_FILE_NAME: &str = "agent-devex.toml";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AgentDevexConfig {
    /// Default Stellar network name for `deploy` (overridden by `--network`).
    #[serde(default)]
    pub network: Option<String>,
}

/// Load config if `dir/agent-devex.toml` exists; `Ok(None)` if it does not.
pub fn load_optional(dir: &Path) -> Result<Option<AgentDevexConfig>, AgentDevexError> {
    let path = dir.join(CONFIG_FILE_NAME);
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).map_err(|source| AgentDevexError::IoError {
        path: path.clone(),
        source,
    })?;
    let cfg = toml::from_str(&raw).map_err(|source| AgentDevexError::InvalidToml {
        path,
        source: Box::new(source),
    })?;
    Ok(Some(cfg))
}
