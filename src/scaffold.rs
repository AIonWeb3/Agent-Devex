//! Scaffold generated projects from compile-time embedded templates.

use std::path::Path;

use crate::Lang;
use crate::embed;
use crate::errors::AgentDevexError;
use crate::paths;

/// Writes all project files and templates to the target directory.
pub fn write_project(root: &Path, project_name: &str, lang: Lang) -> Result<(), AgentDevexError> {
    embed::extract_embedded("project", root, project_name, lang)?;
    embed::extract_embedded("shared", root, project_name, lang)?;
    embed::extract_embedded(
        "soroban",
        &root
            .join(paths::CONTRACTS_DIR)
            .join(paths::AGENT_PAY_CONTRACT),
        project_name,
        lang,
    )?;

    let server = root.join(paths::SERVER_DIR);
    let agent = root.join(paths::AGENT_DIR);
    match lang {
        Lang::Ts => {
            embed::extract_embedded("node-mcp", &server, project_name, lang)?;
            embed::extract_embedded("agent/ts", &agent, project_name, lang)?;
        }
        Lang::Py => {
            embed::extract_embedded("python-mcp", &server, project_name, lang)?;
            embed::extract_embedded("agent/py", &agent, project_name, lang)?;
        }
    }
    Ok(())
}
