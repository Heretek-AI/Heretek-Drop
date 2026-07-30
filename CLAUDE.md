# Heretek-Drop — AI Agent Harness (Strict Mode)

> **Version**: 1.0 — 2026-07-30  
> **Applies to**: Claude Code, OpenCode, and all AI coding agents  
> **Enforcement**: Pre-commit hooks + CI pipelines — see `AI_HARNESS_PLAN.md`

---

## Project Identity

Native AGPL-3.0 client for [Drop](https://github.com/Drop-OSS/drop) — a game distribution platform. Rust + Slint. Linux flatpak v0.1.

## Language Choice (NON-NEGOTIABLE)

- **Default language**: Rust (edition 2024, stable toolchain, 1.85 MSRV)
- **UI framework**: Slint (`.slint` files, compiled to Rust via `slint-build`)
- **NO JavaScript**. NO webview. NO Tauri. NO Chromium. NO Electron.
- Lack of Rust/Slint familiarity is NOT a reason to write code in another language.
- Every `.rs` file MUST start with `// SPDX-License-Identifier: AGPL-3.0` header.

## Currency Enforcement (MANDATORY — do not skip)

Before proposing ANY change that touches dependencies, architecture, or patterns:

1. **VERIFY** current crate versions on crates.io or docs.rs — do not guess or assume versions
2. **CHECK** `Cargo.toml` workspace dependencies — that is the single source of truth
3. **CONFIRM** the Rust edition (2024) and MSRV (1.85) before proposing edition-dependent patterns
4. **SEARCH** `.opencode/skills/` for existing knowledge files before creating new ones
5. **READ** existing code before refactoring — do not guess at its content

**If you are uncertain about a crate version, API shape, or Rust standard**: `WebFetch` the current docs or `Bash` a version query on crates.io. Do not hallucinate API signatures.

## Verification Gate (Self-Check)

Before every non-trivial action, run through this checklist:

- [ ] Did I READ the existing code (not guess)?
- [ ] Did I CHECK docs.rs/crates.io for current API shapes?
- [ ] Did I VERIFY the Rust edition and MSRV match workspace config (`Cargo.toml`)?
- [ ] Did I CHECK `.opencode/skills/` for existing coverage?
- [ ] Did I REVIEW `deny.toml` license allowlist before adding dependencies?
- [ ] Does the change add `unwrap()` or `expect()` in production code? (If yes, STOP — use `?` or error types instead)

## Build & Test (from existing config)

```bash
# Format
cargo fmt --all -- --check

# Lint (must pass cleanly)
cargo clippy --workspace --all-targets -- -D warnings

# Build
cargo build --workspace

# Test
cargo test --workspace

# Docs
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace
```

**You MUST run the toolchain in this order**: fmt → clippy → build → test → doc.

## Anti-Pattern Blocklist (DO NOT DO)

- ❌ "Optimize this" without benchmarks or profiling evidence
- ❌ "Refactor this" without test coverage to validate behavior preservation
- ❌ "Modernize this" without checking what "modern" means for **this** Rust edition (2024)
- ❌ Adding `#[allow(...)]` WITHOUT a commented justification explaining why
- ❌ `unwrap()` or `expect()` in any code path touching network, filesystem, user input, config, or FFI
- ❌ Adding new dependencies without checking `deny.toml` license allowlist AND `cargo deny check`
- ❌ Blocking `std::sync::Mutex` in async code (use `tokio::sync`)
- ❌ Reference to Tauri, webview, JavaScript, Chromium, or Electron in any context
- ❌ Touching `upstream/drop/` — it is a read-only mirror at `cc3f6455`

## Rust Coding Standards

- **Errors**: `thiserror` for library crates, `anyhow::Result` for binary/app glue. NO `unwrap()`.
- **Async**: Tokio multi-thread runtime. Bounded channels only (`flume::bounded(N)` or `tokio::sync::mpsc`).
- **Logging**: `tracing` with structured fields. NEVER `println!` in production code.
- **No unsafe**: Workspace lint denies it. If truly required: wrap in safe API + `// SAFETY:` comment.
- **Borrowing**: Prefer `&str` over `&String`, `&[T]` over `&Vec<T>`. Minimize `clone()`.
- **Naming**: Follow Rust naming conventions (`snake_case` for functions/variables, `CamelCase` for types).

## AGPL Compliance

- All source files MUST include SPDX AGPL-3.0 header: `// SPDX-License-Identifier: AGPL-3.0`
- Static linking upstream Rust crates returns the entire binary to AGPL-3.0 (this is intended)
- The `NOTICE` file MUST credit upstream Drop-OSS authors
- See `.opencode/skills/agpl-compliance/SKILL.md` for full rules

## Documentation Rules

- All public API items need doc comments (`///` or `//!`)
- `cargo doc` must pass with `-D warnings` — broken intra-doc links are CI failures
- ADRs go in `docs/adr/` — record any architectural decision
- The crate dependency table in `README.md` must match workspace members

## Agent-Specific Rules

- **AGENTS.md is root authority**: If CLAUDE.md conflicts with AGENTS.md, AGENTS.md wins. This file extends AGENTS.md, does not override it.
- **New dependencies**: Discuss with user before adding — no unilateral dependency additions
- **Upstream**: `upstream/drop/` is read-only reference at `cc3f6455`. NEVER sync, cherry-pick, or merge from it
- **Repository**: This is `Heretek-AI/Heretek-Drop` — do not confuse with upstream `Drop-OSS/drop`

## When You Are Uncertain

1. READ the existing code first
2. CHECK docs.rs for current API documentation
3. SEARCH `.opencode/skills/` for domain knowledge files
4. Search `upstream/drop/` for prior art (read-only reference)
5. If still uncertain — ASK the user rather than hallucinating an answer

---

_This file is part of the AI Harness system. See `AI_HARNESS_PLAN.md` for the full design and execution roadmap._
