//! Error types and handling for the Agent-Devex CLI.
//!
//! Uses `thiserror` to define typed, structured errors for programmatic handling
//! and user-friendly display.
//!
//! Library code should prefer [`Result`] in this module so failures stay
//! `AgentDevexError` until the binary converts them at the process edge.
use std::path::PathBuf;
use std::process::ExitStatus;

/// Result alias bound to [`AgentDevexError`].
pub type Result<T> = std::result::Result<T, AgentDevexError>;

/// Structured failures for the Agent-Devex CLI and library.
///
/// Variants are displayed via `thiserror` (`Display`) and can be converted
/// into `anyhow::Error` at the binary edge. Prefer returning this enum over
/// panicking for expected operational failures.
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

    #[error("failed to parse config {}: {source}", .path.display())]
    ConfigParseError {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },

    #[error("failed to serialize configuration: {0}")]
    ConfigSerializationError(String),

    #[error("invalid config value for {key}: {value}")]
    InvalidConfigValue { key: String, value: String },

    #[error("validation failed: {0}")]
    ValidationFailed(String),
}
