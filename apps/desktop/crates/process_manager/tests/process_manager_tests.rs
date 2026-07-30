// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Unit tests for the process manager crate.

use std::collections::HashMap;
use std::path::PathBuf;

use heretek_drop_process_manager::{LaunchSpec, ProcessManager};

/// LaunchSpec debug formatting.
#[test]
fn launch_spec_debug() {
    let spec = LaunchSpec {
        id: 1,
        title: "Test Game".into(),
        executable: PathBuf::from("/usr/bin/true"),
        working_dir: None,
        args: vec![],
        env: HashMap::new(),
    };
    let _debug = format!("{spec:?}");
}

/// ProcessManager is Clone and Default.
#[test]
fn process_manager_clone_default() {
    let pm1 = ProcessManager::default();
    let _pm2 = pm1.clone();
}

/// Listing processes on empty manager returns empty list.
#[tokio::test]
async fn list_empty() {
    let pm = ProcessManager::new();
    let procs = pm.list().await;
    assert!(procs.is_empty());
}

/// Querying a non-existent process returns None.
#[tokio::test]
async fn get_non_existent_returns_none() {
    let pm = ProcessManager::new();
    let proc = pm.get(9999).await;
    assert!(proc.is_none());
}

/// LaunchSpec serialization round-trip.
#[test]
fn launch_spec_serde_roundtrip() {
    let spec = LaunchSpec {
        id: 42,
        title: "Portal 2".into(),
        executable: PathBuf::from("/usr/games/portal2"),
        working_dir: Some(PathBuf::from("/opt/portal2")),
        args: vec!["-fullscreen".into(), "-novid".into()],
        env: vec![("HOME".into(), "/home/user".into())]
            .into_iter()
            .collect(),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let deserialized: LaunchSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, 42);
    assert_eq!(deserialized.title, "Portal 2");
    assert_eq!(deserialized.args, vec!["-fullscreen", "-novid"]);
}
