//! Configuration management for Agent-Devex.
//!
//! Handles loading and parsing of the optional `agent-devex.toml` file
//! in the working directory using `serde` and `toml`, plus project-level
//! `agent.toml` ([`AGENT_TOML_FILE_NAME`]).
use std::fs;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::Lang;
use crate::errors::AgentDevexError;

pub const CONFIG_FILE_NAME: &str = "agent-devex.toml";

/// Project configuration file written by `init` and read by compile/deploy/run.
pub const AGENT_TOML_FILE_NAME: &str = "agent.toml";

/// Default local MCP listen port used when `agent.toml` omits a value.
pub const DEFAULT_MCP_PORT: u16 = 3000;

/// Safe development network for newly initialized projects.
pub const DEFAULT_NETWORK: &str = "testnet";

/// Project-level configuration stored in [`AGENT_TOML_FILE_NAME`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Stellar network name (`testnet` or `mainnet`).
    pub network: String,
    /// Optional on-chain contract identifiers (never secret keys).
    pub contract_ids: ContractIds,
    /// Local MCP server settings.
    pub mcp: McpConfig,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            network: DEFAULT_NETWORK.to_string(),
            contract_ids: ContractIds { agent_pay: None },
            mcp: McpConfig::default(),
        }
    }
}

impl AgentConfig {
    /// Resolve `agent.toml` under `project_dir` (defaults to the current directory when `.`).
    pub fn path(project_dir: &Path) -> PathBuf {
        project_dir.join(AGENT_TOML_FILE_NAME)
    }

    /// Read the raw TOML bytes from `agent.toml`.
    pub fn read_raw(project_dir: &Path) -> crate::errors::Result<String> {
        let path = Self::path(project_dir);
        if !path.is_file() {
            return Err(AgentDevexError::ConfigNotFound { path });
        }
        fs::read_to_string(&path).map_err(|source| AgentDevexError::IoError { path, source })
    }

    /// Deserialize `agent.toml` into [`AgentConfig`].
    pub fn load(project_dir: &Path) -> crate::errors::Result<Self> {
        let raw = Self::read_raw(project_dir)?;
        toml::from_str(&raw).map_err(|source| AgentDevexError::ConfigParseError {
            path: Self::path(project_dir),
            source: Box::new(source),
        })
    }
}

/// Named contract ids for a generated project.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContractIds {
    /// AgentPay / agent_pay_integration contract id when known.
    pub agent_pay: Option<String>,
}

/// MCP-related project settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpConfig {
    /// MCP language hint (`ts` or `py`) when set.
    pub lang: Option<String>,
    /// Local MCP HTTP/SSE port.
    pub port: u16,
}

impl McpConfig {
    /// Reject port `0` (unspecified / invalid for a listen address).
    pub fn validate_port(port: u16) -> crate::errors::Result<u16> {
        if port == 0 {
            return Err(AgentDevexError::InvalidPort {
                port: u32::from(port),
            });
        }
        Ok(port)
    }
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            lang: None,
            port: DEFAULT_MCP_PORT,
        }
    }
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
