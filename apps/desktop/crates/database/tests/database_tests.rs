// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Unit tests for the database crate.

use tempfile::TempDir;

use heretek_drop_database::Database;

/// In-memory database round-trip.
#[tokio::test]
async fn in_memory_write_read_delete() {
    let db = Database::open_memory().unwrap();

    // Write
    db.write("key1", &"value1".to_string()).await.unwrap();

    // Read
    let val: Option<String> = db.read("key1").await.unwrap();
    assert_eq!(val, Some("value1".to_string()));

    // Read missing key
    let absent: Option<String> = db.read("key_absent").await.unwrap();
    assert_eq!(absent, None);

    // Overwrite
    db.write("key1", &"value2".to_string()).await.unwrap();
    let val: Option<String> = db.read("key1").await.unwrap();
    assert_eq!(val, Some("value2".to_string()));

    // Delete
    db.delete("key1").await.unwrap();
    let val: Option<String> = db.read("key1").await.unwrap();
    assert_eq!(val, None);
}

/// Multiple keys coexist.
#[tokio::test]
async fn in_memory_multiple_keys() {
    let db = Database::open_memory().unwrap();
    db.write("a", &1_i32).await.unwrap();
    db.write("b", &2_i32).await.unwrap();
    db.write("c", &3_i32).await.unwrap();

    assert_eq!(db.read::<i32>("a").await.unwrap(), Some(1));
    assert_eq!(db.read::<i32>("b").await.unwrap(), Some(2));
    assert_eq!(db.read::<i32>("c").await.unwrap(), Some(3));
}

/// Complex struct serialization round-trip.
#[tokio::test]
async fn in_memory_struct_roundtrip() {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Preferences {
        theme: String,
        max_downloads: u32,
        auto_start: bool,
    }

    let db = Database::open_memory().unwrap();
    let prefs = Preferences {
        theme: "dark".into(),
        max_downloads: 3,
        auto_start: false,
    };
    db.write("prefs", &prefs).await.unwrap();

    let loaded: Preferences = db.read("prefs").await.unwrap().unwrap();
    assert_eq!(loaded, prefs);
}

/// Opening a database creates the file.
#[tokio::test]
async fn on_disk_open_creates_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.db.json");
    let db = Database::open(&path).unwrap();
    db.write("hello", &"world".to_string()).await.unwrap();

    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("hello"));
    assert!(content.contains("world"));
}

/// Opening a non-existent directory creates it.
#[tokio::test]
async fn on_disk_creates_parent_dir() {
    let dir = TempDir::new().unwrap();
    let nested = dir.path().join("a").join("b").join("test.db.json");
    Database::open(&nested).unwrap();
    assert!(nested.parent().unwrap().exists());
}

/// Opening corrupt data starts fresh (graceful degradation).
#[tokio::test]
async fn on_disk_corrupt_file_starts_fresh() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("corrupt.db.json");
    std::fs::write(&path, "not valid json{{{").unwrap();

    let db = Database::open(&path).unwrap();
    let val: Option<String> = db.read("anything").await.unwrap();
    assert_eq!(val, None);

    db.write("after_corrupt", &"ok".to_string()).await.unwrap();
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("after_corrupt"));
}

/// Test default path derivation.
#[test]
fn default_db_path_returns_path() {
    let path = heretek_drop_database::default_db_path();
    assert!(path.is_ok());
    let p = path.unwrap();
    assert!(p.ends_with("db.json"));
    assert!(p.to_string_lossy().contains("heretek") || p.to_string_lossy().contains("drop"));
    assert!(p.to_string_lossy().ends_with("db.json"));
}
