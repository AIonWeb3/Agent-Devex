//! Library root for Agent-Devex CLI.

pub mod commands;
pub mod config;
pub mod errors;
pub mod scaffold;

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Lang {
    Ts,
    Py,
}
