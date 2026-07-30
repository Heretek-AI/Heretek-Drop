// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Cached snapshots — library, user profile, settings.

use serde::{Deserialize, Serialize};

/// Cached library snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedLibrary {
    /// User ID this library belongs to.
    pub user_id: String,
    /// Cached games.
    pub games: Vec<serde_json::Value>,
    /// When this cache was last refreshed.
    pub fetched_at: chrono::DateTime<chrono::Utc>,
}

/// High-level cache API.
#[derive(Debug, Clone)]
pub struct Cache {
    db: crate::Database,
}

impl Cache {
    /// Construct a cache backed by the given database.
    pub fn new(db: crate::Database) -> Self {
        Self { db }
    }

    /// Get the cached library.
    pub async fn library(&self) -> anyhow::Result<Option<CachedLibrary>> {
        Ok(self.db.read("library").await?)
    }

    /// Set the cached library.
    pub async fn set_library(&self, lib: &CachedLibrary) -> anyhow::Result<()> {
        Ok(self.db.write("library", lib).await?)
    }
}
