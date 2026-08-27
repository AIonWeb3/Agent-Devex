//! Typed failure cases for the Agent-Devex CLI.

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

    #[error("{label} failed to start — is stellar-cli installed and on PATH?")]
    StellarSpawn {
        label: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{label} exited with {status}")]
    StellarFailed { label: String, status: ExitStatus },

    #[error("no .wasm after build — check stellar contract build output")]
    WasmNotFound,

    #[error("invalid TOML config {}: {source}", .path.display())]
    InvalidToml {
        path: PathBuf,
        #[source]
        source: Box<toml::de::Error>,
    },
}
