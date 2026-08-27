//! Scaffold generated projects from compile-time template files.
//!
//! Prefer [`include_str!`] over in-source string literals so the Soroban contract and MCP
//! stubs can be edited, reviewed, and syntax-highlighted as their native languages.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::Lang;

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| parent.display().to_string())?;
    }
    fs::write(path, contents).with_context(|| path.display().to_string())
}

fn subst(template: &str, project_name: &str) -> String {
    template.replace("{{PROJECT_NAME}}", project_name)
}

pub fn write_project(root: &Path, project_name: &str, lang: Lang) -> Result<()> {
    write_file(
        &root.join("README.md"),
        &subst(include_str!("../templates/project/README.md"), project_name),
    )?;

    write_file(
        &root
            .join("contracts")
            .join("agent_pay_integration")
            .join("Cargo.toml"),
        include_str!("../templates/contracts/agent_pay_integration/Cargo.toml"),
    )?;
    write_file(
        &root
            .join("contracts")
            .join("agent_pay_integration")
            .join("src")
            .join("lib.rs"),
        include_str!("../templates/contracts/agent_pay_integration/src/lib.rs"),
    )?;

    match lang {
        Lang::Ts => write_agent_ts(root, project_name)?,
        Lang::Py => write_agent_py(root, project_name)?,
    }
    Ok(())
}

fn write_agent_ts(root: &Path, project_name: &str) -> Result<()> {
    let agent = root.join("agent");
    write_file(
        &agent.join("package.json"),
        &subst(
            include_str!("../templates/agent/ts/package.json"),
            project_name,
        ),
    )?;
    write_file(
        &agent.join("tsconfig.json"),
        include_str!("../templates/agent/ts/tsconfig.json"),
    )?;
    write_file(
        &agent.join("README.md"),
        &subst(
            include_str!("../templates/agent/ts/README.md"),
            project_name,
        ),
    )?;
    write_file(
        &agent.join("src").join("index.ts"),
        include_str!("../templates/agent/ts/src/index.ts"),
    )?;
    Ok(())
}

fn write_agent_py(root: &Path, project_name: &str) -> Result<()> {
    let agent = root.join("agent");
    write_file(
        &agent.join("pyproject.toml"),
        &subst(
            include_str!("../templates/agent/py/pyproject.toml"),
            project_name,
        ),
    )?;
    write_file(
        &agent.join("README.md"),
        &subst(
            include_str!("../templates/agent/py/README.md"),
            project_name,
        ),
    )?;
    write_file(
        &agent.join("src").join("server.py"),
        include_str!("../templates/agent/py/src/server.py"),
    )?;
    write_file(&agent.join("src").join("__init__.py"), "")?;
    Ok(())
}
