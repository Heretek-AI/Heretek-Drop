# Architecture Decision Records

This directory contains Architecture Decision Records (ADR) for the Heretek-Drop project.

Each ADR documents a significant architectural decision, its context, consequences, and compliance mechanisms.

## ADR Index

| ADR                                               | Status   | Title                                            | Summary                                                                                                 |
| ------------------------------------------------- | -------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------- |
| [0001](0001-use-husky-9-for-git-hooks.md)         | Accepted | Use Husky 9 for Git Hooks                        | Formalizes existing Husky 9 hook setup, overriding prior Lefthook decision                              |
| [0002](0002-ci-must-check-cargo-doc.md)           | Accepted | CI Must Check `cargo doc` for Broken Links       | Adds `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace` to CI to catch broken intra-doc links |
| [0003](0003-claude-md-must-be-under-200-lines.md) | Accepted | CLAUDE.md Must Be <200 Lines with Currency Rules | Caps agent-facing config at 200 lines with currency enforcement rules for agent adherence               |
