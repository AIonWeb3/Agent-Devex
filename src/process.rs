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
        .map_err(|source| AgentDevexError::ToolSpawn {
            program: program.to_string(),
            label: label.to_string(),
            source,
        })?;
    if !status.success() {
        return Err(AgentDevexError::ToolFailed {
            label: label.to_string(),
            status,
        });
    }
    Ok(status)
}

/// Run a command and return combined stdout/stderr text on success.
pub fn run_in_dir_output(
    program: &str,
    args: &[&str],
    cwd: &Path,
    label: &str,
) -> Result<String, AgentDevexError> {
    tracing::debug!(program, label, cwd = %cwd.display(), "running external command (capture)");
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|source| AgentDevexError::ToolSpawn {
            program: program.to_string(),
            label: label.to_string(),
            source,
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!(%stderr, "command failed");
        return Err(AgentDevexError::ToolFailed {
            label: label.to_string(),
            status: output.status,
        });
    }
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }
    Ok(text)
}

/// Convenience wrapper for `stellar` on PATH (inherit stdio).
pub fn run_stellar(args: &[&str], cwd: &Path, label: &str) -> Result<(), AgentDevexError> {
    run_in_dir("stellar", args, cwd, label)?;
    Ok(())
}

/// Run stellar-cli and capture stdout for parsing (e.g. contract ids).
pub fn run_stellar_output(
    args: &[&str],
    cwd: &Path,
    label: &str,
) -> Result<String, AgentDevexError> {
    run_in_dir_output("stellar", args, cwd, label)
}

/// Best-effort parse of a Soroban contract id (`C` + 55 more base32 chars).
pub fn parse_contract_id(output: &str) -> Option<String> {
    output.split_whitespace().find_map(|token| {
        let t = token.trim_matches(|c| c == '"' || c == '\'' || c == ',');
        if t.len() == 56 && t.starts_with('C') && t.chars().all(|c| c.is_ascii_alphanumeric()) {
            Some(t.to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::parse_contract_id;

    #[test]
    fn parses_contract_id_token() {
        let id = format!("C{}", "A".repeat(55));
        let sample = format!("Contract deployed:\n{id}");
        assert_eq!(parse_contract_id(&sample).as_deref(), Some(id.as_str()));
    }
}
