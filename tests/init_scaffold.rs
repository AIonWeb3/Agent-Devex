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
    assert!(root.join("agent-devex.toml").is_file());
    assert!(root.join(".env.example").is_file());
    let toml = fs::read_to_string(root.join("agent-devex.toml")).unwrap();
    assert!(toml.contains("default_lang = \"ts\""));
    let pkg = fs::read_to_string(root.join("agent/package.json")).unwrap();
    assert!(pkg.contains("demo_agent-agent"));
}
