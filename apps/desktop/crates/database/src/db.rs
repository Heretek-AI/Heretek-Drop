// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Disk-backed database handle. Simple JSON-on-disk kv store.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::error::{DbError, Result};

/// Handle to the database. Cheap to clone.
#[derive(Clone, Debug)]
pub struct Database {
    inner: Arc<Mutex<DatabaseInner>>,
}

struct DatabaseInner {
    /// Path to the on-disk store. None = in-memory only.
    path: Option<PathBuf>,
    /// In-memory cache of the full document.
    cache: Value,
}

// `DatabaseInner` is internal — implement Debug manually so `Database` can derive it
// without exposing internal state.
impl std::fmt::Debug for DatabaseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseInner")
            .field("path", &self.path)
            .field("cache_size_keys", &self.cache.as_object().map(|o| o.len()))
            .finish()
    }
}

impl Database {
    /// Open (or create) a database at the given path.
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let cache = load_from_disk_sync(path)?;
        info!("database opened at {}", path.display());
        Ok(Self {
            inner: Arc::new(Mutex::new(DatabaseInner {
                path: Some(path.to_path_buf()),
                cache,
            })),
        })
    }

    /// Open the database at the default location.
    pub fn open_default() -> Result<Self> {
        let path = default_db_path()?;
        Self::open(&path)
    }

    /// Open an in-memory database (for tests).
    pub fn open_memory() -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(DatabaseInner {
                path: None,
                cache: Value::Object(serde_json::Map::new()),
            })),
        })
    }

    /// Read a JSON value at the given key. Returns `None` if the key is missing.
    pub async fn read<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let inner = self.inner.lock().await;
        let v = inner.cache.get(key).cloned().unwrap_or(Value::Null);
        if v.is_null() {
            return Ok(None);
        }
        let parsed: T = serde_json::from_value(v)?;
        Ok(Some(parsed))
    }

    /// Write a JSON value at the given key.
    pub async fn write<T: serde::Serialize + Sync>(&self, key: &str, value: &T) -> Result<()> {
        let new_value: Value = serde_json::to_value(value)?;
        let mut inner = self.inner.lock().await;
        ensure_object(&mut inner.cache).insert(key.to_string(), new_value);
        flush_to_disk(&inner).await
    }

    /// Delete a key from the database.
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut inner = self.inner.lock().await;
        if let Some(obj) = inner.cache.as_object_mut() {
            obj.remove(key);
        }
        flush_to_disk(&inner).await
    }
}

/// Default database path: `~/.local/share/heretek-drop/db.json` on Linux.
pub fn default_db_path() -> Result<PathBuf> {
    let dirs =
        directories::ProjectDirs::from("dev", "heretek", "drop").ok_or(DbError::NoConfigDir)?;
    Ok(dirs.data_dir().join("db.json"))
}

fn ensure_object(v: &mut Value) -> &mut serde_json::Map<String, Value> {
    if !v.is_object() {
        *v = Value::Object(serde_json::Map::new());
    }
    v.as_object_mut().expect("just set to object")
}

fn load_from_disk_sync(path: &Path) -> Result<Value> {
    if !path.exists() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    let bytes = std::fs::read(path).map_err(DbError::Io)?;
    if bytes.is_empty() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    match serde_json::from_slice(&bytes) {
        Ok(v) => Ok(v),
        Err(e) => {
            warn!("corrupt db file at {}: {e}; starting fresh", path.display());
            Ok(Value::Object(serde_json::Map::new()))
        }
    }
}

async fn flush_to_disk(inner: &DatabaseInner) -> Result<()> {
    let Some(path) = &inner.path else {
        // In-memory mode: nothing to flush.
        return Ok(());
    };
    let bytes = serde_json::to_vec_pretty(&inner.cache)?;
    // Atomic-ish write: write to tmp file, then rename.
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, bytes).map_err(DbError::Io)?;
    std::fs::rename(&tmp, path).map_err(DbError::Io)?;
    Ok(())
}
