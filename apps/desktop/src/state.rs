// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! App-wide shared state.

use std::path::PathBuf;
use std::sync::Arc;

use heretek_drop_auth::Credentials;
use heretek_drop_database::Database;
use heretek_drop_protocol::Client;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::config::Config;

/// App-wide state shared across the Slint UI and Tokio tasks.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    config: Config,
    api: Client,
    db: Database,
    credentials: Option<Credentials>,
    tokio_rt: tokio::runtime::Runtime,
}

impl AppState {
    /// Build app state from config: opens DB, creates HTTP client, sets up runtime.
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let tokio_rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(2)
            .thread_name("heretek-drop")
            .build()?;

        let mut api = Client::new(&config.server.base_url);
        api.set_timeout_secs(config.server.timeout_secs);

        let db_path = db_path()?;
        let db = Database::open(&db_path)?;

        // Load credentials if present (do not fail if absent — user may not have logged in).
        let credentials = Credentials::load_from_default_location().ok().flatten();

        if credentials.is_some() {
            info!("loaded credentials from default location");
        } else {
            warn!("no credentials found; user must log in");
        }

        Ok(Self {
            inner: Arc::new(Mutex::new(Inner {
                config,
                api,
                db,
                credentials,
                tokio_rt,
            })),
        })
    }

    /// Get immutable config.
    pub async fn config(&self) -> Config {
        self.inner.lock().await.config.clone()
    }

    /// Get a clone of the API client.
    pub async fn api(&self) -> Client {
        self.inner.lock().await.api.clone()
    }

    /// Get a clone of the database handle.
    pub async fn db(&self) -> Database {
        self.inner.lock().await.db.clone()
    }

    /// Get current credentials, if logged in.
    pub async fn credentials(&self) -> Option<Credentials> {
        self.inner.lock().await.credentials.clone()
    }

    /// Set credentials (after login).
    pub async fn set_credentials(&self, credentials: Credentials) {
        self.inner.lock().await.credentials = Some(credentials);
    }

    /// Get the tokio runtime handle for spawning tasks.
    pub async fn tokio_handle(&self) -> tokio::runtime::Handle {
        self.inner.lock().await.tokio_rt.handle().clone()
    }
}

fn db_path() -> anyhow::Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "heretek", "drop")
        .ok_or_else(|| anyhow::anyhow!("no config directory available"))?;
    let data_dir = dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&data_dir)?;
    Ok(data_dir.join("db.sqlite"))
}
