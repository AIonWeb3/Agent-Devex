use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_agent-devex"))
}

#[test]
fn help_lists_init_and_compile() {
    let output = cli()
        .arg("--help")
        .output()
        .expect("run agent-devex --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("init"));
    assert!(stdout.contains("compile"));
}
