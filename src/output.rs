//! Terminal output helpers for consistent CLI messaging.

use std::fmt::Display;

/// Print a primary success / completion line to stderr (user-facing).
pub fn success(message: impl Display) {
    eprintln!("{message}");
}

/// Print a follow-up hint / next-step line.
pub fn hint(message: impl Display) {
    eprintln!("{message}");
}

/// Print a warning that does not abort the command.
pub fn warn(message: impl Display) {
    eprintln!("{message}");
}
