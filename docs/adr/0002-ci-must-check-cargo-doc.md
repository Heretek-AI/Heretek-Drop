# ADR-0002: CI Must Check `cargo doc` for Broken Links

## Status

Accepted

## Context

The Rust toolchain's `cargo doc` can detect broken intra-doc links (`broken_intra_doc_links`), missing documentation for public items (`missing_docs`), and invalid code block attributes. However, `cargo check` and `cargo build` do not catch these issues — they are rustdoc-only diagnostics. Without explicit `cargo doc` verification in CI, broken links in documentation silently accumulate, degrading developer and agent experience.

The workspace already has `rustdoc.missing_docs = "warn"` in `[workspace.lints]`, but warnings do not fail CI without `-D warnings`.

## Decision

Add a `cargo doc` job to `ci.yml` that runs with `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace`. This promotes all rustdoc warnings (including `broken_intra_doc_links`, `missing_docs`, and `invalid_codeblock_attributes`) to hard errors in CI.

## Consequences

- Positive: Catches broken intra-doc links before they reach main branch
- Positive: Enforces the workspace `rustdoc.missing_docs = "warn"` lint as a CI gate
- Positive: Prevents documentation decay that would confuse AI coding agents
- Positive: No additional dependencies — uses built-in `cargo doc`
- Neutral: Adds ~1-2 minutes to CI pipeline (mitigated by Swatinem/rust-cache)
- Negative: May cause initial CI failures if existing docs have issues, requiring a cleanup pass

## Compliance

Enforced by CI job in `.github/workflows/ci.yml`:

```yaml
- name: Check doc integrity
  run: RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace
```

The pre-push hook does not run `cargo doc` (too slow for local workflow), so CI is the sole enforcement point.
