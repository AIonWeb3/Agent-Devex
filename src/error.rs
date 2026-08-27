//! Typed errors for scaffolding and deploy helpers (`thiserror`).
//!
//! `main` still returns [`anyhow::Result`] so command-level context can bubble with `?`.

use std::path::PathBuf;
use std::process::ExitStatus;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to create or write {}", .path.display())]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot read directory {}", .path.display())]
    ReadDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("directory {} already exists and is not empty", .path.display())]
    DirectoryNotEmpty { path: PathBuf },
    #[error(
        "missing {} — run `agent-devex init` first or pass --project-dir",
        .path.display()
    )]
    MissingContractManifest { path: PathBuf },
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
}
