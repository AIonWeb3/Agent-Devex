//! Scaffold generated projects from compile-time template files.
//!
//! Prefer [`include_str!`] over in-source string literals so the Soroban contract and MCP
//! stubs can be edited, reviewed, and syntax-highlighted as their native languages.

use std::path::Path;

use crate::Lang;
use crate::errors::AgentDevexError;
use crate::fsutil;

/// Substitutes template variables (`{{PROJECT_NAME}}`, `{{DEFAULT_LANG}}`).
fn subst(template: &str, project_name: &str, lang: Lang) -> String {
    template
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{DEFAULT_LANG}}", lang.as_config_str())
}

/// Writes all project files and templates to the target directory.
pub fn write_project(root: &Path, project_name: &str, lang: Lang) -> Result<(), AgentDevexError> {
    fsutil::write_file(
        &root.join("README.md"),
        &subst(
            include_str!("../templates/project/README.md"),
            project_name,
            lang,
        ),
    )?;
    fsutil::write_file(
        &root.join(".env.example"),
        include_str!("../templates/project/.env.example"),
    )?;
    fsutil::write_file(
        &root.join(".gitignore"),
        include_str!("../templates/project/.gitignore"),
    )?;
    fsutil::write_file(
        &root.join("agent-devex.toml"),
        &subst(
            include_str!("../templates/project/agent-devex.toml"),
            project_name,
            lang,
        ),
    )?;

    fsutil::write_file(
        &root
            .join(crate::paths::CONTRACTS_DIR)
            .join(crate::paths::AGENT_PAY_CONTRACT)
            .join("Cargo.toml"),
        include_str!("../templates/contracts/agent_pay_integration/Cargo.toml"),
    )?;
    fsutil::write_file(
        &root
            .join(crate::paths::CONTRACTS_DIR)
            .join(crate::paths::AGENT_PAY_CONTRACT)
            .join("src")
            .join("lib.rs"),
        include_str!("../templates/contracts/agent_pay_integration/src/lib.rs"),
    )?;

    match lang {
        Lang::Ts => write_agent_ts(root, project_name, lang)?,
        Lang::Py => write_agent_py(root, project_name, lang)?,
    }
    Ok(())
}

fn write_agent_ts(root: &Path, project_name: &str, lang: Lang) -> Result<(), AgentDevexError> {
    let agent = root.join(crate::paths::AGENT_DIR);
    fsutil::write_file(
        &agent.join("package.json"),
        &subst(
            include_str!("../templates/agent/ts/package.json"),
            project_name,
            lang,
        ),
    )?;
    fsutil::write_file(
        &agent.join("tsconfig.json"),
        include_str!("../templates/agent/ts/tsconfig.json"),
    )?;
    fsutil::write_file(
        &agent.join("README.md"),
        &subst(
            include_str!("../templates/agent/ts/README.md"),
            project_name,
            lang,
        ),
    )?;
    fsutil::write_file(
        &agent.join("src").join("index.ts"),
        include_str!("../templates/agent/ts/src/index.ts"),
    )?;
    Ok(())
}

fn write_agent_py(root: &Path, project_name: &str, lang: Lang) -> Result<(), AgentDevexError> {
    let agent = root.join(crate::paths::AGENT_DIR);
    fsutil::write_file(
        &agent.join("pyproject.toml"),
        &subst(
            include_str!("../templates/agent/py/pyproject.toml"),
            project_name,
            lang,
        ),
    )?;
    fsutil::write_file(
        &agent.join("README.md"),
        &subst(
            include_str!("../templates/agent/py/README.md"),
            project_name,
            lang,
        ),
    )?;
    fsutil::write_file(
        &agent.join("src").join("server.py"),
        include_str!("../templates/agent/py/src/server.py"),
    )?;
    fsutil::write_file(&agent.join("src").join("__init__.py"), "")?;
    Ok(())
}
