//! Small filesystem helpers used by scaffolding and command implementations.

use std::fs;
use std::path::Path;

use crate::errors::AgentDevexError;

/// True when `path` does not exist, or exists and contains no entries.
pub fn is_missing_or_empty_dir(path: &Path) -> Result<bool, AgentDevexError> {
    if !path.exists() {
        return Ok(true);
    }
    if !path.is_dir() {
        return Ok(false);
    }
    Ok(path
        .read_dir()
        .map_err(|source| AgentDevexError::IoError {
            path: path.to_path_buf(),
            source,
        })?
        .next()
        .is_none())
}

/// Write `contents` to `path`, creating parent directories as needed.
pub fn write_file(path: &Path, contents: &str) -> Result<(), AgentDevexError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| AgentDevexError::IoError {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, contents).map_err(|source| AgentDevexError::IoError {
        path: path.to_path_buf(),
        source,
    })
}

/// Write bytes to `path`, creating parent directories as needed.
pub fn write_bytes(path: &Path, contents: &[u8]) -> Result<(), AgentDevexError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| AgentDevexError::IoError {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, contents).map_err(|source| AgentDevexError::IoError {
        path: path.to_path_buf(),
        source,
    })
}
