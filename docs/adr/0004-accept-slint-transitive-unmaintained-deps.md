# ADR-0004: Accept transitive Slint unmaintained-deps warnings

## Status

Accepted. Supersedes inline deny.toml comments for RUSTSEC-2025-0141 (bincode), RUSTSEC-2026-0206 (rustybuzz), and RUSTSEC-2026-0192 (ttf-parser). RUSTSEC-2024-0436 (paste) was already accepted informally; this ADR formalizes the policy.

## Context

`cargo audit` reports 4 unmaintained-crate warnings as of 2026-07-30:

| Advisory          | Crate      | Version | Path                                                     |
| ----------------- | ---------- | ------- | -------------------------------------------------------- |
| RUSTSEC-2024-0436 | paste      | 1.0.15  | Slint transitive                                         |
| RUSTSEC-2025-0141 | bincode    | 2.0.1   | Slint → typed-index-collections → bincode                |
| RUSTSEC-2026-0206 | rustybuzz  | 0.20.1  | Slint → usvg → resvg → rustybuzz                         |
| RUSTSEC-2026-0192 | ttf-parser | 0.25.1  | Slint → winit → ab_glyph → owned_ttf_parser → ttf-parser |

None have CVEs. All are "unmaintained" warnings — the projects are functionally complete but no longer receive updates. No direct usage in heretek-drop; all enter the dependency tree through Slint 1.17.1.

GitHub Dependabot surfaces these as "moderate" severity on the default branch.

## Decision

Accept all four warnings. Do not block releases on unmaintained-but-functional transitive deps. Re-evaluate at v0.2 stable cut (target: 2026-12-31).

## Consequences

**Trade-offs:**

- Pre-commit/CI gate (`cargo deny check`) stays clean.
- Dependabot alerts remain visible on the repo but are documented as accepted.
- If a CVE lands against any of these crates, this ADR must be revisited immediately.

**Why not fork/replace:**

- bincode 2.x is the live major; replacement would require dropping Slint or patching Slint's internals. Out of scope for v0.1.
- rustybuzz and ttf-parser are font-rendering primitives deeply embedded in Slint's software renderer path.
- No drop-in replacement maintains the same rendering behavior.

**Risk:**

- Low. These are feature-complete crates with no active security exposure.
- Mitigation: `cargo audit` runs in CI on schedule (cargo-audit.yml, planned in phase 3) so a real CVE would be caught within days.

## Compliance

- `deny.toml` `[advisories].ignore` lists all four with structured `{id, reason}` entries.
- Pre-commit hook runs `cargo fmt → clippy → fallow → doc-check → prettier`; CI `cargo-audit` job will run weekly + on Cargo.lock change (Phase 3).
- Security risk register at `security/risk-register.yaml` references this ADR.
- Quarterly review: check whether Slint upstream has released a version with refreshed dependencies.
