use anyhow::Result;
use std::path::PathBuf;

use crate::errors::AgentDevexError;
use crate::fsutil;
use crate::output;
use crate::{Lang, scaffold};

pub fn cmd_init(project_name: &str, lang: Lang) -> Result<()> {
    let root = PathBuf::from(project_name);
    if !fsutil::is_missing_or_empty_dir(&root)? {
        return Err(AgentDevexError::DirectoryNotEmpty { path: root }.into());
    }

    scaffold::write_project(&root, project_name, lang)?;

    output::success(format!("Created {project_name}/"));
    output::hint("  contracts/agent_pay_integration  Soroban + AgentPay/AgentGuard");
    match lang {
        Lang::Ts => {
            output::hint("  agent/                          TypeScript MCP server");
            output::hint(format!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            ));
            output::hint("      cd agent && npm install && npx tsx src/index.ts");
        }
        Lang::Py => {
            output::hint("  agent/                          Python MCP server");
            output::hint(format!(
                "Next: cd {project_name} && stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml"
            ));
            output::hint("      cd agent && uv sync && uv run python src/server.py");
        }
    }
    Ok(())
}
