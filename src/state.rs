//! Local project state (not secrets) stored under `.agent-devex/`.

use std::fs;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::errors::AgentDevexError;
use crate::fsutil;

pub const STATE_DIR: &str = ".agent-devex";
pub const STATE_FILE: &str = "state.toml";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProjectState {
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub contract_id: Option<String>,
    #[serde(default)]
    pub wasm_path: Option<String>,
    #[serde(default)]
    pub last_deployed_unix: Option<u64>,
}

pub fn state_path(project_dir: &Path) -> PathBuf {
    project_dir.join(STATE_DIR).join(STATE_FILE)
}

pub fn load_optional(project_dir: &Path) -> Result<Option<ProjectState>, AgentDevexError> {
    let path = state_path(project_dir);
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).map_err(|source| AgentDevexError::IoError {
        path: path.clone(),
        source,
    })?;
    let state = toml::from_str(&raw).map_err(|source| AgentDevexError::InvalidToml {
        path,
        source: Box::new(source),
    })?;
    Ok(Some(state))
}

pub fn save(project_dir: &Path, state: &ProjectState) -> Result<(), AgentDevexError> {
    let path = state_path(project_dir);
    let raw =
        toml::to_string_pretty(state).map_err(|source| AgentDevexError::InvalidConfigValue {
            key: STATE_FILE.to_string(),
            value: source.to_string(),
        })?;
    fsutil::write_file(&path, &raw)
}

#[cfg(test)]
mod tests {
    use super::ProjectState;

    #[test]
    fn roundtrip_toml() {
        let state = ProjectState {
            network: Some("testnet".into()),
            contract_id: Some(format!("C{}", "B".repeat(55))),
            wasm_path: Some("out.wasm".into()),
            last_deployed_unix: Some(1),
        };
        let encoded = toml::to_string(&state).unwrap();
        let decoded: ProjectState = toml::from_str(&encoded).unwrap();
        assert_eq!(state, decoded);
    }
}
