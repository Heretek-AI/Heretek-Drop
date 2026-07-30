// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Library service — fetches and caches the user's game library.

use crate::events::{Event, EventBus};
use crate::state::AppState;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Cached library snapshot.
#[derive(Debug, Clone, Serialize)]
pub struct CachedLibrary {
    /// User ID.
    pub user_id: String,
    /// Cached games.
    pub games: Vec<heretek_drop_protocol::Game>,
}

/// Library service handle.
#[derive(Clone)]
pub struct LibraryService {
    state: AppState,
    bus: EventBus,
    cache: Arc<Mutex<Option<CachedLibrary>>>,
}

impl LibraryService {
    /// Create a new library service.
    #[must_use]
    pub fn new(state: AppState, bus: EventBus) -> Self {
        Self {
            state,
            bus,
            cache: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the cached library, if any.
    pub async fn cached(&self) -> Option<CachedLibrary> {
        self.cache.lock().await.clone()
    }

    /// Refresh the library from the Drop server.
    pub async fn refresh(&self) -> Result<usize, crate::AppError> {
        info!("refreshing library");

        let credentials = self.state.credentials().await.ok_or_else(|| {
            crate::AppError::Auth(heretek_drop_auth::AuthError::Config("not logged in".into()))
        })?;

        let api = self.state.api().await;
        let auth_ctx = heretek_drop_protocol::AuthContext {
            private: &credentials.private,
            certificate: &credentials.certificate,
            client_id: &credentials.id,
        };
        let games = api
            .user_library(&auth_ctx)
            .await
            .map_err(crate::AppError::Protocol)?;
        let count = games.len();
        info!(count, "library fetched");

        let lib = CachedLibrary {
            user_id: credentials.id.clone(),
            games,
        };
        *self.cache.lock().await = Some(lib);

        // Persist to disk.
        if let Ok(json) = serde_json::to_value(self.cache.lock().await.clone()) {
            let db = self.state.db().await;
            if let Err(e) = db.write("library", &json).await {
                warn!("failed to persist library cache: {e}");
            }
        }

        self.bus.try_send(Event::LibraryLoaded { count });
        Ok(count)
    }

    /// Load cached library from disk into memory.
    pub async fn load_cache(&self) -> Result<(), crate::AppError> {
        let db = self.state.db().await;
        match db.read::<serde_json::Value>("library").await {
            Ok(Some(_)) => {
                info!("loaded library cache from disk");
            }
            Ok(None) => info!("no library cache on disk"),
            Err(e) => warn!("failed to load library cache: {e}"),
        }
        Ok(())
    }

    /// Resolve a game ID to a `VersionDownloadOption` (latest).
    pub async fn resolve_download(
        &self,
        game_id: u32,
    ) -> Result<heretek_drop_protocol::VersionDownloadOption, crate::AppError> {
        let credentials = self.state.credentials().await.ok_or_else(|| {
            crate::AppError::Auth(heretek_drop_auth::AuthError::Config("not logged in".into()))
        })?;
        let api = self.state.api().await;
        let auth_ctx = heretek_drop_protocol::AuthContext {
            private: &credentials.private,
            certificate: &credentials.certificate,
            client_id: &credentials.id,
        };
        let versions = api
            .game_versions(game_id, &auth_ctx)
            .await
            .map_err(crate::AppError::Protocol)?;
        versions.into_iter().next().ok_or_else(|| {
            crate::AppError::Protocol(heretek_drop_protocol::ProtocolError::Config(format!(
                "no versions for game {game_id}"
            )))
        })
    }
}
