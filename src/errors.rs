//! Error types and handling for the Agent-Devex CLI.
//!
//! Uses `thiserror` to define typed, structured errors for programmatic handling
//! and user-friendly display.
use std::path::PathBuf;
use std::process::ExitStatus;

#[derive(Debug, thiserror::Error)]
pub enum AgentDevexError {
    /// Missing project or tool config (e.g. generated `Cargo.toml`, future CLI config).
    #[error("config not found: {}", .path.display())]
    ConfigNotFound { path: PathBuf },

    #[error("I/O error at {}: {source}", .path.display())]
    IoError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("directory {} already exists and is not empty", .path.display())]
    DirectoryNotEmpty { path: PathBuf },

    #[error("{label} failed to start — is `{program}` installed and on PATH?")]
    ToolSpawn {
        program: String,
        label: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{label} exited with {status}")]
    ToolFailed { label: String, status: ExitStatus },

    #[error("required tool `{program}` was not found on PATH")]
    ToolMissing { program: String },

    #[error("invalid project name `{name}`: {reason}")]
    InvalidProjectName { name: String, reason: String },

    #[error("no .wasm after build — check stellar contract build output")]
    WasmNotFound,

    #[error("invalid TOML config {}: {source}", .path.display())]
    InvalidToml {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },

    #[error("invalid config value for {key}: {value}")]
    InvalidConfigValue { key: String, value: String },

    #[error("validation failed: {0}")]
    ValidationFailed(String),
}
