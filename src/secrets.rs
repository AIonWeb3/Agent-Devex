//! Helpers so secrets never appear in logs or error messages.

/// Mask Stellar secret keys and other high-entropy tokens for display.
pub fn redact(value: &str) -> String {
    let v = value.trim();
    if v.starts_with('S') && v.len() >= 32 && v.chars().all(|c| c.is_ascii_alphanumeric()) {
        return format!("{}… (redacted)", &v[..4]);
    }
    if v.contains("SECRET") || v.contains("PRIVATE") {
        return "(redacted)".to_string();
    }
    v.to_string()
}

/// True when an environment variable name should never be printed with its value.
pub fn is_secret_env(name: &str) -> bool {
    let n = name.to_ascii_uppercase();
    n.contains("SECRET") || n.contains("PRIVATE") || n.ends_with("_KEY")
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn redacts_stellar_secret_shape() {
        let key = format!("S{}", "X".repeat(55));
        let out = redact(&key);
        assert!(!out.contains(&key));
        assert!(out.contains("redacted"));
    }
}
