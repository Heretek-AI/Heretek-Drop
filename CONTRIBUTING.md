# Contributing to Heretek-Drop

## Quick Start

```bash
# Prerequisites
sudo apt-get install -y flatpak flatpak-builder
rustup toolchain install stable
cargo install slint-viewer

# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Run
cargo run --bin heretek-drop

# Live UI iteration
slint-viewer apps/desktop/src/ui/app-window.slint
```

## Development Setup

1. Fork and clone the repo
2. Create a branch: `git checkout -b feat/my-feature`
3. Make changes
4. Run the pre-commit hook: `cargo fmt && cargo clippy`
5. Commit using conventional commits: `feat(download): add pause/resume`
6. Push and open a PR against `main`

## Commit Convention

We use conventional commits with scopes. Every commit message must be:

```
<type>(<scope>): <subject>

<body> (optional)
<footer> (optional)
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

Scopes: `desktop`, `protocol`, `auth`, `db`, `download`, `process`, `ui`, `lint`, `deps`, `release`, `ci`, `agents`, `skills`

Enforced by `commitlint` via Husky `commit-msg` hook.

## PR Process

1. Ensure tests pass: `cargo test --workspace`
2. Ensure lints pass: `cargo clippy --workspace --all-targets -- -D warnings`
3. Ensure formatting: `cargo fmt --all -- --check`
4. Squash commits to a single logical commit per change
5. PR title matches commit convention
6. Add at least one reviewer

## Coding Standards

- **Rust edition**: 2024, stable toolchain
- **All source files**: `// SPDX-License-Identifier: AGPL-3.0` header
- **No `unsafe`**: workspace lint denies it
- **No `unwrap()` in prod code**: use `anyhow::Result` or `thiserror`
- **Async**: Tokio multi-thread runtime
- **Logging**: `tracing`, never `println!`
- **Errors**: `thiserror` for libraries, `anyhow!` for app glue

## AGPL Compliance

This project is AGPL-3.0. All contributions are accepted under the
same license. By contributing, you agree that your code will be
licensed AGPL-3.0.

- Every `.rs` file must start with a SPDX header
- No AGPL-incompatible dependencies
- `NOTICE` file credits upstream authors
- `cargo deny` CI check enforces license compliance

## Project Architecture

```
Heretek-Drop/
├── apps/desktop/              # Binary: heretek-drop (Slint + Rust)
│   ├── src/                   # Binary + Slint UI
│   ├── crates/shared/        # Cross-crate error types
│   ├── crates/protocol/      # Drop REST API client
│   ├── crates/auth/          # Auth flow + credential persistence
│   ├── crates/database/      # JSON-on-disk key-value store
│   ├── crates/download_manager/  # Chunked download engine
│   ├── crates/process_manager/   # Game process lifecycle
│   ├── flatpak/              # Flatpak build manifest
│   └── icons/                # App icons (SVG + PNG)
└── .github/workflows/        # CI/CD workflows
```

## Dependencies

No third-party Rust crates beyond what is declared in `Cargo.toml`
and workspace `[workspace.dependencies]`. No JavaScript, no webview,
no Tauri.

## Getting Help

- Open an issue for bugs or feature requests
- See `apps/desktop/TESTING.md` for manual smoke test steps
- See `.omo/plans/drop-native-client-decisions.md` for architecture decisions
