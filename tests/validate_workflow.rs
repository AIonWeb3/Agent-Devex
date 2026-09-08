use agent_devex::commands::validate::cmd_validate;
use agent_devex::scaffold::write_project;
use agent_devex::Lang;

#[test]
fn validate_accepts_fresh_python_project() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("py_agent");
    write_project(&root, "py_agent", Lang::Py).unwrap();
    cmd_validate(&root).unwrap();
    assert!(root.join("agent/src/server.py").is_file());
}

#[test]
fn validate_rejects_empty_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let err = cmd_validate(tmp.path()).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("validation failed") || msg.contains("missing"));
}
