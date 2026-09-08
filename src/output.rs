//! Terminal output helpers for consistent CLI messaging.

use std::fmt::Display;
use std::io::IsTerminal;

use colored::Colorize;

fn color_enabled() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stderr().is_terminal()
}

/// Print a primary success / completion line to stderr (user-facing).
pub fn success(message: impl Display) {
    let text = message.to_string();
    if color_enabled() {
        eprintln!("{} {text}", "✓".green().bold());
    } else {
        eprintln!("ok {text}");
    }
}

/// Print a follow-up hint / next-step line.
pub fn hint(message: impl Display) {
    let text = message.to_string();
    if color_enabled() {
        eprintln!("{}", text.dimmed());
    } else {
        eprintln!("{text}");
    }
}

/// Print a warning that does not abort the command.
pub fn warn(message: impl Display) {
    let text = message.to_string();
    if color_enabled() {
        eprintln!("{} {text}", "!".yellow().bold());
    } else {
        eprintln!("warn {text}");
    }
}

/// Print a hard failure line (commands still return Err separately).
pub fn error(message: impl Display) {
    let text = message.to_string();
    if color_enabled() {
        eprintln!("{} {text}", "x".red().bold());
    } else {
        eprintln!("error {text}");
    }
}
