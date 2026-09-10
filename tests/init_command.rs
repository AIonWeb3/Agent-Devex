use std::fs;
use std::sync::Mutex;

use agent_devex::Lang;
use agent_devex::commands::init::cmd_init;
use agent_devex::config::AgentConfig;

static CWD: Mutex<()> = Mutex::new(());

#[test]
fn init_creates_directory_and_agent_toml() {
    let _cwd = CWD.lock().unwrap_or_else(|e| e.into_inner());
    let tmp = tempfile::tempdir().unwrap();
    let orig = std::env::current_dir().unwrap();
    std::env::set_current_dir(tmp.path()).unwrap();
    let result = cmd_init("demo_init", Some(Lang::Ts));
    let _ = std::env::set_current_dir(&orig);
    result.unwrap();

    let root = tmp.path().join("demo_init");
    assert!(root.is_dir());
    let cfg = AgentConfig::load(&root).unwrap();
    assert_eq!(cfg.mcp.lang.as_deref(), Some("ts"));
}

#[test]
fn init_fails_when_project_exists() {
    let _cwd = CWD.lock().unwrap_or_else(|e| e.into_inner());
    let tmp = tempfile::tempdir().unwrap();
    let orig = std::env::current_dir().unwrap();
    std::env::set_current_dir(tmp.path()).unwrap();
    fs::create_dir_all("taken").unwrap();
    fs::write("taken/marker", "x").unwrap();
    let result = cmd_init("taken", Some(Lang::Ts));
    let _ = std::env::set_current_dir(&orig);
    let err = result.unwrap_err();
    assert!(err.to_string().contains("already exists"));
}
