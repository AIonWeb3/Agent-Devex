# Current CLI error handling flow

## Process edge

`src/main.rs` returns `anyhow::Result<()>`. Clap parse failures exit via Clap (not `AgentDevexError`). Recoverable command failures should return `Err` from handlers so `main` prints the error and uses a non-zero exit code. Unexpected panics are reserved for programming bugs and are handled by `human-panic` in release builds.

## Domain errors

`AgentDevexError` (`src/errors.rs`, `thiserror`) is the typed error used across the library:

- **Config / I/O** — `ConfigNotFound`, `IoError`, `InvalidToml`, `InvalidConfigValue`
- **Scaffolding** — `DirectoryNotEmpty`, `InvalidProjectName`
- **Tooling** — `ToolSpawn`, `ToolFailed`, `ToolMissing`
- **Deploy** — `WasmNotFound`
- **Validate** — `ValidationFailed`

Handlers typically `?` convert `AgentDevexError` into `anyhow::Error` with `.into()`.

## Intended extensions

Add explicit variants for config parse vs serialize, invalid deploy network, invalid MCP port, project-already-exists, and project-creation failures so callers can match without string inspection. Keep `From<std::io::Error>` (or path-aware wrapping) so `?` works at I/O boundaries. Do not replace `anyhow` at `main`.
