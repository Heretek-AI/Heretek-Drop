#!/usr/bin/env sh
# Copyright (C) 2025 Heretek-Drop contributors
# SPDX-License-Identifier: AGPL-3.0
#
# doc-check.sh -- Public API doc-comment warning for pre-commit hooks
# Source this file, then call check_doc_comments().
#
# check_doc_comments() scans staged .rs files for new public items
# (pub fn, pub struct, pub enum, pub trait, pub mod, pub type, pub const)
# that lack a preceding doc comment (/// or //!). Warns only, does not fail.
#
# Detection uses git diff --cached -U1 with awk heuristic -- simplified,
# not a full AST parse.

check_doc_comments() {
    total=0

    staged_rs=$(git diff --cached --name-only --diff-filter=ACMR -- '*.rs' 2>/dev/null || true)
    [ -z "$staged_rs" ] && return 0

    for file in $staged_rs; do
        output=$(git diff --cached -U1 -- "$file" 2>/dev/null | \
            awk '
                /^\+(pub fn|pub struct|pub enum|pub trait|pub mod|pub type|pub const)/ {
                    if (prev !~ /\/\/[\/!]/ && prev !~ /^---/) {
                        gsub(/^\+/, "", $0);
                        printf "%s\n", $0;
                    }
                }
                { prev = $0 }
            ' || true)

        if [ -n "$output" ]; then
            count=$(echo "$output" | wc -l)
            echo "$output" | while IFS= read -r pub_line; do
                printf "  [DOC] %s -- %s\n" "$file" "$pub_line"
            done
            total=$((total + count))
        fi
    done

    if [ $total -gt 0 ]; then
        printf "  -> %d public item(s) undocumented. Add doc comments (/// or //!).\n" "$total"
    fi
    return 0
}
