//! Subprocess helpers for invoking external tooling (stellar-cli, etc.).

use std::path::Path;
use std::process::{Command, ExitStatus};

use crate::errors::AgentDevexError;

/// Run `program` with `args` in `cwd`.
///
/// Distinguishes spawn failures (binary missing) from non-zero exits.
pub fn run_in_dir(
    program: &str,
    args: &[&str],
    cwd: &Path,
    label: &str,
) -> Result<ExitStatus, AgentDevexError> {
    tracing::debug!(program, label, cwd = %cwd.display(), "running external command");
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .map_err(|source| AgentDevexError::StellarSpawn {
            label: label.to_string(),
            source,
        })?;
    if !status.success() {
        return Err(AgentDevexError::StellarFailed {
            label: label.to_string(),
            status,
        });
    }
    Ok(status)
}

/// Convenience wrapper for `stellar` on PATH.
pub fn run_stellar(args: &[&str], cwd: &Path, label: &str) -> Result<(), AgentDevexError> {
    run_in_dir("stellar", args, cwd, label)?;
    Ok(())
}
