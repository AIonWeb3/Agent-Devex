use agent_devex::Lang;
use agent_devex::scaffold::write_project;
use std::fs;

#[test]
fn init_ts_writes_contract_and_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("demo_agent");
    write_project(&root, "demo_agent", Lang::Ts).unwrap();

    assert!(
        root.join("contracts/agent_pay_integration/src/lib.rs")
            .is_file()
    );
    assert!(root.join("agent/src/index.ts").is_file());
    assert!(root.join("server/src/index.ts").is_file());
    assert!(root.join("agent-devex.toml").is_file());
    assert!(root.join(".env.example").is_file());
    let toml = fs::read_to_string(root.join("agent-devex.toml")).unwrap();
    assert!(toml.contains("default_lang = \"ts\""));
    let pkg = fs::read_to_string(root.join("agent/package.json")).unwrap();
    assert!(pkg.contains("demo_agent-agent"));
}

#[test]
fn init_outputs_structured_directory_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("test-proj");
    write_project(&root, "test-proj", Lang::Py).unwrap();

    for path in [
        "contracts/agent_pay_integration/Cargo.toml",
        "contracts/agent_pay_integration/src/lib.rs",
        "contracts/agent_pay_integration/src/test.rs",
        "contracts/agent_pay_integration/Makefile",
        "server/main.py",
        "server/requirements.txt",
        "agent/src/server.py",
        "README.md",
        ".gitignore",
        ".env.example",
    ] {
        assert!(root.join(path).is_file(), "missing {path}");
    }

    let readme = fs::read_to_string(root.join("README.md")).unwrap();
    assert!(readme.contains("test-proj"));
    assert!(!readme.contains("{{PROJECT_NAME}}"));

    let lib = fs::read_to_string(root.join("contracts/agent_pay_integration/src/lib.rs")).unwrap();
    assert!(lib.contains("fn deposit"));
    assert!(lib.contains("fn execute_payment"));
    assert!(lib.contains("fn refund"));
}
