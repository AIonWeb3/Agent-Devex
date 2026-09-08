use anyhow::Result;
use std::path::PathBuf;

use crate::errors::AgentDevexError;
use crate::{Lang, scaffold};

pub fn cmd_init(project_name: &str, lang: Lang) -> Result<()> {
    let root = PathBuf::from(project_name);
    if root.exists() {
        let empty = root
            .read_dir()
            .map_err(|source| AgentDevexError::IoError {
                path: root.clone(),
                source,
            })?
            .next()
            .is_none();
        if !empty {
            return Err(AgentDevexError::DirectoryNotEmpty { path: root }.into());
        }
    }

    scaffold::write_project(&root, project_name, lang)?;

    eprintln!("Created {project_name}/");
    eprintln!("  contracts/agent_pay_integration  Soroban + AgentPay/AgentGuard");
    match lang {
        Lang::Ts => {
            eprintln!("  agent/                          TypeScript MCP server");
            eprintln!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            );
            eprintln!("      cd agent && npm install && npx tsx src/index.ts");
        }
        Lang::Py => {
            eprintln!("  agent/                          Python MCP server");
            eprintln!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            );
            eprintln!("      cd agent && uv sync && uv run python src/server.py");
        }
    }
    Ok(())
}
