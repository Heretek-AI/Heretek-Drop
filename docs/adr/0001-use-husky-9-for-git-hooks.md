# ADR-0001: Use Husky 9 for Git Hooks

## Status

Accepted

## Context

The project needs consistent git hook management for pre-commit (fmt → clippy → fallow → prettier), commit-msg (conventional commits via commitlint), and pre-push (cargo test) enforcement. An earlier decision specified Lefthook, but the implementation used Husky 9, which is already deployed and working.

Actual files in use:

- `.husky/pre-commit` — runs fmt, clippy, fallow, prettier
- `.husky/commit-msg` — runs commitlint for conventional commit enforcement
- `.husky/pre-push` — runs `cargo test --workspace --verbose`
- `package.json` includes `"prepare": "husky"` for automated install

## Decision

Adopt Husky 9 as the git hook manager. This formalizes the existing implementation and overrides the prior Lefthook decision. Husky 9 is stable, well-maintained (2.5k+ GitHub stars), and the project's hooks are already functional.

## Consequences

- Positive: No migration cost — hooks already work with Husky 9
- Positive: Husky 9 has native npm lifecycle integration via `prepare`
- Positive: Community familiarity with Husky (wider adoption than Lefthook)
- Neutral: Dynamic crate-level hook targeting is a future enhancement on top of Husky
- Negative: Lefthook's per-crate configuration isn't available without a custom script

## Compliance

Enforced by:

- `.husky/` directory tracked in git as the single source of hook truth
- `package.json` "prepare": "husky" ensures hooks install on `npm install` / `pnpm install`
- CI does not enforce hook content (hooks are local), but the pre-push hook calls `cargo test` which CI also runs
