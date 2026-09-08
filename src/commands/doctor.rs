use anyhow::Result;
use std::process::Command;

use crate::output;

struct Check {
    name: &'static str,
    program: &'static str,
    args: &'static [&'static str],
    required: bool,
    hint: &'static str,
}

const CHECKS: &[Check] = &[
    Check {
        name: "Rustc",
        program: "rustc",
        args: &["--version"],
        required: true,
        hint: "Install via https://rustup.rs/",
    },
    Check {
        name: "Cargo",
        program: "cargo",
        args: &["--version"],
        required: true,
        hint: "Install via https://rustup.rs/",
    },
    Check {
        name: "stellar-cli",
        program: "stellar",
        args: &["--version"],
        required: false,
        hint: "https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli",
    },
    Check {
        name: "Node.js",
        program: "node",
        args: &["--version"],
        required: false,
        hint: "Required for --lang ts MCP servers. https://nodejs.org/",
    },
    Check {
        name: "npm",
        program: "npm",
        args: &["--version"],
        required: false,
        hint: "Ships with Node.js; used to install the TypeScript agent.",
    },
    Check {
        name: "Python",
        program: "python",
        args: &["--version"],
        required: false,
        hint: "Required for --lang py (python3 is also accepted).",
    },
    Check {
        name: "uv",
        program: "uv",
        args: &["--version"],
        required: false,
        hint: "https://docs.astral.sh/uv/ — recommended for Python MCP servers.",
    },
];

pub fn cmd_doctor() -> Result<()> {
    output::success("Agent-Devex toolchain doctor");
    let mut missing_required = 0usize;
    for check in CHECKS {
        match probe(check.program, check.args) {
            Some(version) => {
                output::hint(format!("  ok   {:<12} {version}", check.name));
            }
            None if check.program == "python" => {
                if let Some(version) = probe("python3", check.args) {
                    output::hint(format!("  ok   {:<12} {version}", check.name));
                } else {
                    report_missing(check, &mut missing_required);
                }
            }
            None => report_missing(check, &mut missing_required),
        }
    }

    if missing_required > 0 {
        anyhow::bail!(
            "{missing_required} required tool(s) missing — install them before building the CLI"
        );
    }
    output::success(
        "Required tools are present. Optional gaps only affect deploy or a specific --lang.",
    );
    Ok(())
}

fn report_missing(check: &Check, missing_required: &mut usize) {
    let kind = if check.required { "need" } else { "skip" };
    output::hint(format!(
        "  {kind} {:<12} not found — {}",
        check.name, check.hint
    ));
    if check.required {
        *missing_required += 1;
    }
}

fn probe(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let mut text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        text = String::from_utf8_lossy(&output.stderr).trim().to_string();
    }
    text.lines().next().map(str::to_string)
}
