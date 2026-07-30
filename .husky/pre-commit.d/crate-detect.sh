#!/usr/bin/env sh
# Copyright (C) 2025 Heretek-Drop contributors
# SPDX-License-Identifier: AGPL-3.0
#
# crate-detect.sh — Dynamic crate detection for pre-commit hooks
# Source this file, then call detect_changed_crates().
#
# detect_changed_crates() examines git diff --cached and maps file paths
# to workspace crate -p arguments for targeted cargo commands.
#
# Returns:
#   0 — partial crate change; stdout contains space-separated -p arguments
#   1 — full workspace change (root Cargo.toml touched); stdout is empty
#
# Usage:
#   CRATES=$(detect_changed_crates) || FULL_WORKSPACE=1

detect_changed_crates() {
    changed_files=$(git diff --cached --name-only --diff-filter=ACMR 2>/dev/null || true)
    [ -z "$changed_files" ] && return 0

    crates=""
    for file in $changed_files; do
        case "$file" in
            Cargo.toml|apps/desktop/Cargo.toml)
                return 1
                ;;
            apps/desktop/crates/auth/*)
                crates="$crates -p heretek-drop-auth"
                ;;
            apps/desktop/crates/database/*)
                crates="$crates -p heretek-drop-database"
                ;;
            apps/desktop/crates/protocol/*)
                crates="$crates -p heretek-drop-protocol"
                ;;
            apps/desktop/crates/download_manager/*)
                crates="$crates -p heretek-drop-download-manager"
                ;;
            apps/desktop/crates/process_manager/*)
                crates="$crates -p heretek-drop-process-manager"
                ;;
            apps/desktop/crates/shared/*)
                crates="$crates -p heretek-drop-shared"
                ;;
            apps/desktop/src/*)
                crates="$crates -p heretek-drop"
                ;;
        esac
    done

    if [ -n "$crates" ]; then
        echo "$crates" | tr ' ' '\n' | sed '/^$/d' | sort -u | tr '\n' ' '
    fi
    return 0
}
