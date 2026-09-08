//! Validation for user-supplied project identifiers.

use crate::errors::AgentDevexError;

/// Accept directory names that work on Windows and Unix and as cargo package names.
pub fn validate_project_name(name: &str) -> Result<(), AgentDevexError> {
    if name.is_empty() {
        return Err(AgentDevexError::InvalidProjectName {
            name: name.to_string(),
            reason: "name must not be empty".to_string(),
        });
    }
    if name == "." || name == ".." {
        return Err(AgentDevexError::InvalidProjectName {
            name: name.to_string(),
            reason: "use a dedicated directory name, not `.` or `..`".to_string(),
        });
    }
    if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
        return Err(AgentDevexError::InvalidProjectName {
            name: name.to_string(),
            reason: "name contains characters that are not valid in a directory".to_string(),
        });
    }
    let first = name.chars().next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err(AgentDevexError::InvalidProjectName {
            name: name.to_string(),
            reason: "name should start with a letter or underscore".to_string(),
        });
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(AgentDevexError::InvalidProjectName {
            name: name.to_string(),
            reason: "use only letters, digits, hyphens, and underscores".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_project_name;

    #[test]
    fn accepts_simple_names() {
        validate_project_name("my-agent").unwrap();
        validate_project_name("demo_01").unwrap();
    }

    #[test]
    fn rejects_paths_and_empty() {
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("../secret").is_err());
        assert!(validate_project_name("has space").is_err());
        assert!(validate_project_name("9lives").is_err());
        assert!(validate_project_name("-lead").is_err());
    }
}
