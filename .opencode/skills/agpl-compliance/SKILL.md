---
name: agpl-compliance
description: AGPL-3.0 obligations for Heretek-Drop. Source headers, NOTICE, static linking rules. Use when licensing concerns arise.
---
> **Currency**: This skill was last verified against the workspace dependencies in `Cargo.toml` (slint 1.13, tokio 1.42, reqwest 0.12, jsonwebtoken 9.3, thiserror 2.0, anyhow 1.0). Before relying on API shapes from this file, check current docs.rs for the relevant crate. If versions have drifted, update this note or refresh the skill content.


# AGPL-3.0 Compliance — Heretek-Drop

Heretek-Drop is licensed AGPL-3.0 because:
1. It links upstream Drop Rust crates (`droplet`, `libarchive`, `native_model`) statically.
2. Static linking any AGPL-3.0 code forces the entire binary to AGPL-3.0.

## License header (REQUIRED)

Every Rust source file MUST start with:

```rust
// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0
```

Enforce via Husky pre-commit.

## LICENSE file

`/LICENSE` is a pointer to the full AGPL-3.0 text:

```
AGPL-3.0.LICENSE
```

`/AGPL-3.0.LICENSE` is the verbatim GNU AGPL v3 text (661 lines).

## NOTICE file

Required for crediting upstream authors. Format:

```
Heretek-Drop
Copyright (C) 2025 Heretek-Drop contributors
SPDX-License-Identifier: AGPL-3.0

This product includes software developed by
Drop-OSS contributors (https://github.com/Drop-OSS/drop).
```

## Static linking rules

| Action | Result |
|---|---|
| Link upstream Rust crate statically | AGPL-3.0 applies to entire binary |
| Call Drop REST API (HTTP only) | AGPL-3.0 still applies if you link any upstream code |
| Vendor upstream Rust source verbatim | AGPL-3.0 applies |
| Vendor upstream Rust source, modify | Modified version still AGPL-3.0; must publish |
| No upstream code, only REST calls | Could be MIT, but we keep AGPL-3.0 for cohesion |

## Network use (AGPL §13)

If anyone runs the AGPL-3.0 binary as a network service, they MUST offer the source to users of that service. This applies to:
- Hosting a Drop server (already covered by upstream)
- Any Heretek-Drop-as-a-service offering (deferred, not v0.1)

## Re-licensing path (NOT v0.1)

To move to MIT, you must:
1. Remove all upstream Rust source vendoring
2. Reimplement all linked upstream crates from scratch (clean-room)
3. Verify no Drift via reverse engineering
4. SPDX-License-Identifier changes to MIT
5. Update NOTICE file

Cost: ~3-6 weeks of pure re-implementation. **Not in v0.1 scope.**

## CI enforcement

- `cargo deny` lints check dependencies' licenses
- `LICENSE` and `AGPL-3.0.LICENSE` are git-tracked (NOT in `.gitignore`)
- Husky pre-commit blocks commits without AGPL header

## Don't

- Don't add a `LICENSE-MIT` file or dual-license anything. AGPL-3.0 is the only license.
- Don't vendor upstream Rust code without explicit PR approval.
- Don't add dependencies whose license is incompatible with AGPL-3.0 (per `cargo deny` denylist).
- Don't claim authorship of upstream code — preserve NOTICE.
