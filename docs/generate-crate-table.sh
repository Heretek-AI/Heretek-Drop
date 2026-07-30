#!/usr/bin/env bash
# docs/generate-crate-table.sh
# Regenerates the workspace crate table for documentation.
#
# Reads each workspace member's Cargo.toml and outputs a markdown
# table with (Crate, Purpose) columns. The Purpose column is the
# crate's `description = "..."` field from its manifest.
#
# Usage: bash docs/generate-crate-table.sh
#        pnpm docs:sync
#
# Per AI_HARNESS_PLAN.md phase 5.1.

set -euo pipefail

# Resolve repo root regardless of where this script is invoked from.
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

MEMBER_DIRS=$(find apps/desktop/crates -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort)

if [ -z "$MEMBER_DIRS" ]; then
    echo "ERROR: no workspace members found under apps/desktop/crates/" >&2
    exit 1
fi

echo "| Crate | Purpose |"
echo "| --- | --- |"

for dir in $MEMBER_DIRS; do
    name=$(basename "$dir")
    manifest="$dir/Cargo.toml"

    if [ ! -f "$manifest" ]; then
        echo "WARN: $manifest missing, skipping" >&2
        continue
    fi

    # Cargo package name (kebab-case) maps to the directory name with
    # underscores converted to dashes (project convention).
    crate_name="heretek-drop-${name//_/-}"

    # Extract single-line description. Match start of line, stop at
    # the closing quote.
    description=$(grep -E '^description\s*=\s*"' "$manifest" \
        | head -n 1 \
        | sed -E 's/^description[[:space:]]*=[[:space:]]*"//; s/".*$//')

    if [ -z "$description" ]; then
        description="(no description in $manifest)"
    fi

    echo "| \`$crate_name\` | $description |"
done
