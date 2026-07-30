# Heretek-Drop — Claude Code Compatibility Shim

This file mirrors `AGENTS.md` for Claude Code compatibility. When Claude Code is the active agent, it reads this file in addition to `AGENTS.md`.

## Project

Native client for [Drop](https://github.com/Drop-OSS/drop) — AGPL-3.0 game distribution platform. Rust + Slint. Linux flatpak v0.1.

## Stack

- **Shell**: Rust (edition 2024, stable)
- **UI**: Slint (`.slint` files)
- **Build**: Cargo workspace
- **Distribution**: Linux flatpak

## Don't

- No JavaScript. No webview. No Tauri.
- No upstream sync. `upstream/drop/` is read-only at `cc3f6455`.
- No commits without explicit user request.

See `AGENTS.md` for full architecture, agent precedence, and AGPL compliance rules.
