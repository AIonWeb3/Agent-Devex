//! Compile-time embedding of `templates/` via `rust-embed`.

use std::path::Path;

use rust_embed::RustEmbed;

use crate::Lang;
use crate::errors::AgentDevexError;
use crate::fsutil;

/// All files under the repository `templates/` directory, baked into the CLI binary.
#[derive(RustEmbed)]
#[folder = "templates/"]
#[exclude = "**/target/**"]
pub struct TemplateAsset;

/// Substitutes template variables (`{{PROJECT_NAME}}`, `{{DEFAULT_LANG}}`).
pub fn subst(template: &str, project_name: &str, lang: Lang) -> String {
    template
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{DEFAULT_LANG}}", lang.as_config_str())
}

/// Extract every embedded file whose path starts with `prefix/` into `dest`.
///
/// UTF-8 files receive `{{PROJECT_NAME}}` / `{{DEFAULT_LANG}}` substitution.
/// Missing prefix fails with [`AgentDevexError::TemplateMissing`].
pub fn extract_embedded(
    prefix: &str,
    dest: &Path,
    project_name: &str,
    lang: Lang,
) -> Result<(), AgentDevexError> {
    let prefix = prefix.trim_end_matches('/');
    let needle = format!("{prefix}/");
    let mut found = false;

    for path in TemplateAsset::iter() {
        let path = path.as_ref();
        let Some(rel) = path.strip_prefix(&needle) else {
            continue;
        };
        if rel.is_empty() {
            continue;
        }
        found = true;
        let file = TemplateAsset::get(path)
            .ok_or_else(|| AgentDevexError::TemplateMissing(path.to_string()))?;
        let dest_path = dest.join(rel);
        match std::str::from_utf8(file.data.as_ref()) {
            Ok(text) => fsutil::write_file(&dest_path, &subst(text, project_name, lang))?,
            Err(_) => fsutil::write_bytes(&dest_path, file.data.as_ref())?,
        }
    }

    if !found {
        return Err(AgentDevexError::TemplateMissing(prefix.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_embed_locates_templates_directory() {
        assert!(
            TemplateAsset::get("soroban/Cargo.toml").is_some(),
            "rust-embed must read templates/soroban at compile time"
        );
        assert!(TemplateAsset::get("python-mcp/requirements.txt").is_some());
        assert!(TemplateAsset::get("node-mcp/package.json").is_some());
        assert!(TemplateAsset::get("shared/.env.example").is_some());
    }

    #[test]
    fn extract_writes_substituted_files() {
        let tmp = tempfile::tempdir().unwrap();
        extract_embedded("shared", tmp.path(), "demo_proj", Lang::Ts).unwrap();
        let env = std::fs::read_to_string(tmp.path().join(".env.example")).unwrap();
        assert!(env.contains("STELLAR_RPC_URL"));
        assert!(!env.contains("{{PROJECT_NAME}}"));
    }
}
