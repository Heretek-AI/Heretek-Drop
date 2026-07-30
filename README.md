# Heretek-Drop

A native, AGPL-3.0 client for [Drop](https://github.com/Drop-OSS/drop) — an open-source game distribution platform (like a self-hosted Steam).

## Stack

- **Language**: Rust (edition 2024, stable)
- **UI**: Slint (declarative UI, no webview, no Chromium)
- **Distribution**: Linux flatpak (v0.1)

## Status

**v0.1 (in development)** — minimal feature set: auth, browse library, download, launch.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 heretek-drop                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │ Slint UI │  │ Commands │  │  EventBus     │  │
│  │  .slint  │◄─┤  Bridge  │◄─┤ (flume 256)   │  │
│  └────┬─────┘  └────┬─────┘  └───────┬───────┘  │
│       │             │                │           │
│  ┌────▼─────────────▼────────────────▼───────┐   │
│  │              Services Layer               │   │
│  │  Auth │ Library │ Download │ Settings     │   │
│  └────┬────────┬──────┬──────────┬───────────┘   │
│       │        │      │          │               │
│  ┌────▼────────▼──────▼──────────▼───────────┐   │
│  │              Sub-crates                    │   │
│  │ protocol │ auth │ database │ dl_manager   │   │
│  │               │ process_manager           │   │
│  └───────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
                        │
              HTTP (REST + JWT)
                        │
               ┌────────▼────────┐
               │   Drop Server   │
               │  (upstream)     │
               └─────────────────┘
```

Single Rust binary. Cargo workspace at `apps/desktop/` with sub-crates:

| Crate                           | Purpose                                         |
| ------------------------------- | ----------------------------------------------- |
| `heretek-drop` (root)           | Binary + Slint UI + glue                        |
| `heretek-drop-protocol`         | HTTP client + JWT auth header injection         |
| `heretek-drop-auth`             | Auth flow (initiate, handshake, code, sign-out) |
| `heretek-drop-database`         | JSON-on-disk key-value store                    |
| `heretek-drop-download-manager` | Chunked downloads with SHA-256 verification     |
| `heretek-drop-process-manager`  | Game process tracking                           |
| `heretek-drop-shared`           | Cross-crate error types                         |

## Build

```bash
# Prerequisites
sudo apt-get install -y flatpak flatpak-builder
rustup toolchain install stable
cargo install slint-viewer

# Build
cargo build --workspace

# Run
cargo run --bin heretek-drop

# Tests
cargo test --workspace

# Format + lint
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Live UI iteration (no Rust recompile)
cargo install slint-viewer
slint-viewer apps/desktop/src/ui/app-window.slint

# Flatpak build (v0.1 release target)
cd apps/desktop
flatpak-builder build flatpak/app.heretek.drop.yml --user --install
flatpak run app.heretek.drop
```

## Git Hooks

Pre-commit runs: `cargo fmt → clippy → fallow → prettier`
Commit-msg enforces: conventional commits (via commitlint)
Pre-push runs: `cargo test --workspace`

## Packages

```bash
# Release (from GitHub Releases)
flatpak install ./app.heretek.drop-v0.1.0-x86_64.flatpak
```

Flatpak builds for x86_64 and aarch64 on every tag push.

## Links

- [Contributing](CONTRIBUTING.md)
- [Testing](apps/desktop/TESTING.md)
- [Architecture decisions](.omo/plans/drop-native-client-decisions.md)
- [Agent rules](AGENTS.md)
- [GitHub Issues](https://github.com/Heretek-AI/Heretek-Drop/issues)

## License

AGPL-3.0. See `LICENSE` and `AGPL-3.0.LICENSE`.

Upstream attribution in `NOTICE`.

## Upstream

- `upstream/drop/` is a read-only mirror of [Drop-OSS/drop](https://github.com/Drop-OSS/drop) at commit `cc3f6455`. Reference only, not synced.
- CI + hook patterns ported from [BillyOutlast/drop#200](https://github.com/BillyOutlast/drop/pull/200).
