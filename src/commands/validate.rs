use anyhow::Result;
use std::path::Path;

use crate::config;
use crate::errors::AgentDevexError;
use crate::output;
use crate::paths;

/// Verify a generated project has the expected layout and parseable config.
pub fn cmd_validate(project_dir: &Path) -> Result<()> {
    let mut issues = Vec::new();

    let manifest = paths::contract_manifest(project_dir);
    if !manifest.is_file() {
        issues.push(format!("missing contract manifest {}", manifest.display()));
    }

    let lib = paths::contract_crate_dir(project_dir)
        .join("src")
        .join("lib.rs");
    if !lib.is_file() {
        issues.push(format!("missing contract source {}", lib.display()));
    }

    let agent = paths::agent_dir(project_dir);
    let has_ts =
        agent.join("package.json").is_file() && agent.join("src").join("index.ts").is_file();
    let has_py =
        agent.join("pyproject.toml").is_file() && agent.join("src").join("server.py").is_file();
    if !has_ts && !has_py {
        issues.push(format!(
            "missing MCP server under {} (expected TypeScript or Python layout)",
            agent.display()
        ));
    }

    let readme = project_dir.join("README.md");
    if !readme.is_file() {
        issues.push("missing project README.md".to_string());
    }
    if !project_dir.join(".env.example").is_file() {
        issues.push("missing .env.example".to_string());
    }
    if !project_dir.join(".gitignore").is_file() {
        issues.push("missing .gitignore".to_string());
    }

    if let Some(cfg) = config::load_optional(project_dir)? {
        if let Some(parsed) = cfg.parsed_default_lang() {
            parsed?;
        }
    }

    if !issues.is_empty() {
        return Err(AgentDevexError::ValidationFailed(issues.join("; ")).into());
    }

    let kind = if has_ts { "TypeScript" } else { "Python" };
    output::success(format!(
        "Project {} looks healthy ({kind} MCP + AgentPay contract).",
        project_dir.display()
    ));
    Ok(())
}
