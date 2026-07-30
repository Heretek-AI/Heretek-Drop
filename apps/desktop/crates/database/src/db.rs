// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Disk-backed database handle.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustbreak::{MemoryBackend, PathBackend};
use tokio::sync::Mutex;
use tracing::info;

use crate::error::{DbError, Result};

/// Handle to the database. Cheap to clone.
#[derive(Clone)]
pub struct Database {
    inner: Arc<Mutex<DatabaseInner>>,
}

struct DatabaseInner {
    memory: Option<MemoryBackend>,
    path: Option<PathBuf>,
}

impl Database {
    /// Open (or create) a database at the given path.
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let backend = PathBackend::create(path)?;
        let _: serde_json::Value = backend.load().unwrap_or_default();
        info!("database opened at {}", path.display());
        Ok(Self {
            inner: Arc::new(Mutex::new(DatabaseInner {
                memory: None,
                path: Some(path.to_path_buf()),
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
                memory: Some(MemoryBackend::create()),
                path: None,
            })),
        })
    }

    /// Read a JSON value at the given key. Returns `None` if the key is missing.
    pub async fn read<T: serde::de::DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        let value = self.load_full().await?;
        let v = value.get(key).cloned().unwrap_or(serde_json::Value::Null);
        if v.is_null() {
            return Ok(None);
        }
        let parsed: T = serde_json::from_value(v)?;
        Ok(Some(parsed))
    }

    /// Write a JSON value at the given key.
    pub async fn write<T: serde::Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<()> {
        let new_value: serde_json::Value = serde_json::to_value(value)?;
        let mut current = self.load_full().await?;
        if let Some(obj) = current.as_object_mut() {
            obj.insert(key.to_string(), new_value);
        } else {
            let mut obj = serde_json::Map::new();
            obj.insert(key.to_string(), new_value);
            current = serde_json::Value::Object(obj);
        }
        self.store_full(&current).await
    }

    /// Delete a key from the database.
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut current = self.load_full().await?;
        if let Some(obj) = current.as_object_mut() {
            obj.remove(key);
        }
        self.store_full(&current).await
    }

    async fn load_full(&self) -> Result<serde_json::Value> {
        use rustbreak::Backend;
        let inner = self.inner.lock().await;
        match (&inner.memory, &inner.path) {
            (Some(mem), _) => Ok(mem.load().unwrap_or_default()),
            (None, Some(p)) => {
                let backend = PathBackend::create(p)?;
                Ok(backend.load().unwrap_or_default())
            }
            (None, None) => Err(DbError::Corruption("no backend".into())),
        }
    }

    async fn store_full(&self, value: &serde_json::Value) -> Result<()> {
        use rustbreak::Backend;
        let inner = self.inner.lock().await;
        match (&inner.memory, &inner.path) {
            (Some(mem), _) => {
                mem.write(value.clone())?;
            }
            (None, Some(p)) => {
                let backend = PathBackend::create(p)?;
                backend.write(value.clone())?;
            }
            (None, None) => return Err(DbError::Corruption("no backend".into())),
        }
        Ok(())
    }
}

/// Default database path: `~/.local/share/heretek-drop/db.json` on Linux.
pub fn default_db_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "heretek", "drop")
        .ok_or(DbError::NoConfigDir)?;
    Ok(dirs.data_dir().join("db.json"))
}
