//! Panic reporting metadata for unexpected crashes.
//!
//! Recoverable CLI failures still use [`crate::AgentDevexError`]. This module
//! only prepares diagnostics for true panics.

/// Issue tracker used in panic reports.
pub const PANIC_HOMEPAGE: &str = "https://github.com/AIonWeb3/Agent-Devex";

/// Build `human-panic` metadata (authors, homepage). The hook is installed later.
pub fn panic_metadata() -> human_panic::Metadata {
    human_panic::metadata!()
        .authors("AIonWeb3")
        .homepage(PANIC_HOMEPAGE)
}
