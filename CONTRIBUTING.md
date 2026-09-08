# Contributing to Agent-Devex

Thank you for your interest in contributing to Agent-Devex! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust** (stable, edition 2024) — install via [rustup](https://rustup.rs/)
- **Git** — for version control

### Getting Started

```bash
# Clone the repository
git clone https://github.com/AIonWeb3/Agent-Devex.git
cd Agent-Devex

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

## Development Workflow

1. **Fork** the repository
2. **Create a branch** from `main` with a descriptive name:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. **Make your changes** following the coding standards below
4. **Test** your changes thoroughly
5. **Commit** with a clear, conventional commit message
6. **Push** your branch and open a Pull Request

## Coding Standards

### Rust Code

- Run `cargo fmt --all` before committing
- Run `cargo clippy --all-targets -- -D warnings` and fix all warnings
- Run `cargo test` and ensure all tests pass
- Use `thiserror` for typed errors in library code
- Use `anyhow` for error propagation at the binary edge
- Add doc comments (`///`) for public items
- Use `tracing` for structured logging (not `println!`)

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new feature
fix: resolve bug in module
refactor: restructure code without changing behavior
docs: update documentation
test: add or improve tests
chore: maintenance tasks
perf: performance improvements
```

### Templates

Files in `templates/` are compiled into the binary via `include_str!`. When modifying templates:

- Keep them as valid, syntax-highlightable source files
- Use `{{PROJECT_NAME}}` for project name substitution
- Test scaffolding with `cargo run -- init test-project --lang ts`
- Clean up test projects after testing

## Pull Request Guidelines

- Keep PRs focused on a single change
- Include a clear description of what and why
- Reference related issues if applicable
- Ensure CI passes before requesting review
- Update documentation if your change affects the public API

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Include reproduction steps for bugs
- Include your OS, Rust version, and relevant tool versions

## Code of Conduct

Be respectful, constructive, and inclusive. We're all here to build something great together.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
