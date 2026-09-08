//! Panic reporting metadata for unexpected crashes.
//!
//! Recoverable CLI failures still use [`crate::AgentDevexError`]. This module
//! only prepares diagnostics for true panics.

/// Issue tracker used in panic reports.
pub const PANIC_HOMEPAGE: &str = "https://github.com/AIonWeb3/Agent-Devex";

/// Support line included in the human-panic dump.
pub const PANIC_SUPPORT: &str = "- Open an issue: https://github.com/AIonWeb3/Agent-Devex/issues";

/// Build `human-panic` metadata (authors, homepage, support).
pub fn panic_metadata() -> human_panic::Metadata {
    human_panic::metadata!()
        .authors("AIonWeb3")
        .homepage(PANIC_HOMEPAGE)
        .support(PANIC_SUPPORT)
}

/// Install the process-wide human-panic hook (release dumps; debug keeps default).
pub fn install_panic_handler() {
    human_panic::setup_panic!(panic_metadata());
}
