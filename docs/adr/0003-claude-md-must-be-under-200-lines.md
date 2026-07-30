# ADR-0003: CLAUDE.md Must Be <200 Lines with Currency Rules

## Status

Accepted

## Context

Research shows that longer CLAUDE.md files reduce agent adherence — agents skip or gloss over instructions past a threshold. The current CLAUDE.md is 22 lines (too brief), but the risk is the opposite extreme: unbounded growth as more rules are added.

The file serves as the primary agent briefing and must remain scannable. The plan (section 2.1) establishes a 200-line hard cap, alongside currency enforcement rules (crate version verification, Rust edition checks, skill directory searches) to prevent agents from acting on stale knowledge.

## Decision

Enforce a 200-line maximum for `CLAUDE.md` (and any agent-facing markdown files in the project root). Each file must also include an explicit "Currency enforcement" section with verification steps agents must follow before proposing architectural changes, new dependencies, or refactors.

## Consequences

- Positive: Short files increase agent instruction adherence (research-backed)
- Positive: Forces maintainers to be concise and prioritize essential rules
- Positive: Currency rules prevent agents from guessing stale crate APIs
- Positive: Single-file authority reduces conflicting instructions across files
- Negative: Requires discipline to keep under 200 lines as project grows
- Negative: May need to split into referenced sub-files for complex topics

## Compliance

Enforced by:

- Pre-commit line-count check (future implementation) or manual review during PRs
- The rule itself must be in `CLAUDE.md` as a self-referential enforcement mechanism
- Periodic audits during CLAUDE.md updates
