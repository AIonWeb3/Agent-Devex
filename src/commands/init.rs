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
    crate::names::validate_project_name(project_name)?;
    let root = PathBuf::from(project_name);
    if !fsutil::is_missing_or_empty_dir(&root)? {
        return Err(AgentDevexError::DirectoryNotEmpty { path: root }.into());
    }

    tracing::info!(project = %project_name, ?lang, "scaffolding project");
    scaffold::write_project(&root, project_name, lang)?;
    crate::next_steps::after_init(project_name, lang);
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
