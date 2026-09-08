use anyhow::Result;
use std::io::IsTerminal;
use std::path::PathBuf;

use crate::config;
use crate::errors::AgentDevexError;
use crate::fsutil;
use crate::output;
use crate::{Lang, scaffold};

pub fn cmd_init(project_name: &str, lang: Option<Lang>) -> Result<()> {
    let lang = resolve_lang(lang)?;
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

fn resolve_lang(explicit: Option<Lang>) -> Result<Lang> {
    if let Some(lang) = explicit {
        return Ok(lang);
    }

    if let Some(cfg) = config::load_optional(std::path::Path::new("."))? {
        if let Some(parsed) = cfg.parsed_default_lang() {
            return parsed.map_err(Into::into);
        }
    }

    if std::io::stdin().is_terminal() && std::io::stderr().is_terminal() {
        let items = ["TypeScript (ts)", "Python (py)"];
        let idx = dialoguer::Select::new()
            .with_prompt("MCP server language")
            .items(&items)
            .default(0)
            .interact()?;
        return Ok(if idx == 0 { Lang::Ts } else { Lang::Py });
    }

    output::hint("No --lang provided; defaulting to TypeScript (pass --lang ts|py to choose).");
    Ok(Lang::Ts)
}
