//! Configuration management for Agent-Devex.
//!
//! Handles loading and parsing of the optional `agent-devex.toml` file
//! in the working directory using `serde` and `toml`, plus project-level
//! `agent.toml` ([`AGENT_TOML_FILE_NAME`]).
use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::Lang;
use crate::errors::AgentDevexError;

pub const CONFIG_FILE_NAME: &str = "agent-devex.toml";

/// Project configuration file written by `init` and read by compile/deploy/run.
pub const AGENT_TOML_FILE_NAME: &str = "agent.toml";

/// Project-level configuration stored in [`AGENT_TOML_FILE_NAME`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentConfig {
    /// Stellar network name (`testnet` or `mainnet`).
    pub network: String,
    /// Optional on-chain contract identifiers (never secret keys).
    pub contract_ids: ContractIds,
}

/// Named contract ids for a generated project.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContractIds {
    /// AgentPay / agent_pay_integration contract id when known.
    pub agent_pay: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AgentDevexConfig {
    /// Default Stellar network name for `deploy` (overridden by `--network`).
    #[serde(default)]
    pub network: Option<String>,

    /// Default MCP language when `init` omits `--lang`.
    #[serde(default)]
    pub default_lang: Option<String>,

    #[serde(default)]
    pub project: ProjectMeta,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ProjectMeta {
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl AgentDevexConfig {
    /// Parsed default language, if set and valid (`ts` or `py`).
    pub fn parsed_default_lang(&self) -> Option<Result<Lang, AgentDevexError>> {
        self.default_lang.as_deref().map(parse_lang)
    }
}

fn parse_lang(value: &str) -> Result<Lang, AgentDevexError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "ts" | "typescript" => Ok(Lang::Ts),
        "py" | "python" => Ok(Lang::Py),
        other => Err(AgentDevexError::InvalidConfigValue {
            key: "default_lang".to_string(),
            value: other.to_string(),
        }),
    }
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

/// Load config or return defaults when the file is absent.
pub fn load_or_default(dir: &Path) -> Result<AgentDevexConfig, AgentDevexError> {
    Ok(load_optional(dir)?.unwrap_or_default())
}
