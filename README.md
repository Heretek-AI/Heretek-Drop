# Heretek-Drop

A native, AGPL-3.0 client for [Drop](https://github.com/Drop-OSS/drop) — an open-source game distribution platform (like a self-hosted Steam).

## Stack

- **Language**: Rust (edition 2024, stable)
- **UI**: Slint (declarative UI, no webview)
- **Distribution**: Linux flatpak (v0.1)

## Status

**v0.1 (in development)** — minimal feature set: auth, browse library, download, launch.

## Architecture

Single Rust binary. Cargo workspace at `apps/desktop/` with sub-crates:

| Crate | Purpose |
|---|---|
| `heretek-drop` (root) | Binary + Slint UI + glue |
| `protocol` | HTTP client + JWT auth header injection |
| `auth` | Auth flow (initiate, handshake, code, sign-out) |
| `database` | SQLite via `rustbreak` |
| `download_manager` | Chunked downloads, progress events |
| `process_manager` | Game process tracking |

## Build

```bash
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
```

## License

AGPL-3.0. See `LICENSE` and `AGPL-3.0.LICENSE`.

Upstream attribution in `NOTICE` (TBD).

## Upstream

- `upstream/drop/` is a read-only mirror of [Drop-OSS/drop](https://github.com/Drop-OSS/drop) at commit `cc3f6455`. Reference only, not synced.

## Contributing

TBD. See `AGENTS.md` for agent rules.
