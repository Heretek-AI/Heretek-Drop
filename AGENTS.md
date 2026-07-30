# Heretek-Drop — Agent Architecture

## Project

Native client for [Drop](https://github.com/Drop-OSS/drop) — an open-source AGPL-3.0 game distribution platform. Built in Rust + Slint. Targets Linux flatpak for v0.1.

## Architecture (one paragraph)

Single Rust binary. Slint declarative UI. Talks to Drop server over REST + JWT. Rust workspace at `apps/desktop/` with sub-crates: `protocol`, `auth`, `database`, `download_manager`, `process_manager`. The shell renders `.slint` files; backend logic lives in pure Rust crates invoked via `slint::invoke_from_event_loop` or direct thread calls.

## Locked decisions (Phase 0)

See `.omo/plans/drop-native-client-decisions.md` for full provenance.

| Topic          | Decision                          |
| -------------- | --------------------------------- |
| License        | AGPL-3.0                          |
| Platforms v0.1 | Linux flatpak                     |
| Architecture   | Slint + Rust (no webview)         |
| Scope          | Auth + browse + download + launch |
| CI             | Full matrix on every PR           |
| Repo           | Keep Heretek-Drop                 |
| Namespace      | `app.heretek.drop`                |
| Git hooks      | Husky 9                           |
| Tauri          | NOT USED                          |
| Webview        | NOT USED                          |

## Agent precedence

Project-level `.opencode/` config OVERRIDES user-level `~/.config/opencode/` config. This is the OpenCode default (see OpenCode docs). When a subagent runs, it inherits:

1. Root `AGENTS.md` (this file)
2. Per-folder `AGENTS.md` (when present)
3. `.opencode/agents/*.md` subagent definitions
4. `.opencode/skills/*/SKILL.md` skill definitions
5. Global user config

## Subagents

- `rust-builder` — Rust crates, Tauri-style Cargo workspace, async Tokio, JWT signing
- `ui-builder` — Slint markup, .slint files, component design, accessibility
- `reviewer` — code review focus, security/AGPL compliance, performance

## Skills

- `drop-protocol` — REST API shapes at `/api/v1/client/*`, JWT auth flow
- `slint-ui` — Slint markup syntax, callbacks, component composition
- `agpl-compliance` — AGPL-3.0 obligations, static linking rules, NOTICE file
- `rust-gui-patterns` — Slint + Tokio patterns, event-loop bridging, image loading

## Build commands

```bash
# Dev build
cargo build --workspace

# Run the app
cargo run --bin heretek-drop

# Live UI iteration (no recompile)
slint-viewer apps/desktop/src/ui/app-window.slint

# Test
cargo test --workspace

# Format + lint
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Flatpak
cd apps/desktop && flatpak-builder build flatpak/app.heretek.drop.yml --user --install
```

## AGPL compliance

- All Rust source files MUST include AGPL-3.0 header comment. Enforce via Husky pre-commit.
- `LICENSE` file MUST point to `AGPL-3.0.LICENSE`.
- `NOTICE` file MUST credit upstream Drop-OSS authors.
- Static-linking any upstream Rust crate (`droplet`, `libarchive`, `native_model`) RETURNS the entire binary to AGPL-3.0. Default: link upstream crates freely.
- NO clean-room MIT path for v0.1.

## Upstream tracking

- `upstream/drop/` is a READ-ONLY mirror at `cc3f6455` (Drop-OSS/drop develop).
- We do NOT pull from upstream. We do NOT cherry-pick from upstream.
- The 10 files we vendored from `BillyOutlast/drop#200` (read `.github/CODEOWNERS` for full list) are isolated infrastructure — they don't sync.

## When you don't know

- READ existing code before guessing. Use `codegraph_explore` for symbol search.
- When architecture is unclear, search `upstream/drop/` for prior art.
- When unsure about an API shape, check `.opencode/skills/drop-protocol/SKILL.md`.
- Never `@ts-ignore`-style suppress errors. Never `as any` in Rust. Never unwrap without comment.

## Don't

- Don't write JavaScript. Don't add webview code. Don't reference Tauri.
- Don't add new top-level dependencies without discussion.
- Don't touch `upstream/drop/` — it's read-only.
- Don't commit without an explicit user request.
- Don't push, create PRs, or merge without explicit user request.
