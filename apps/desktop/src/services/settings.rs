// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Settings service — user preferences persisted via the database.

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::events::{Event, EventBus};
use crate::state::AppState;

/// User settings (persisted as a single JSON blob).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Server base URL.
    pub server_url: String,
    /// Downloads directory.
    pub download_dir: String,
    /// Max concurrent downloads.
    pub max_concurrent: u32,
    /// Theme: `auto`, `light`, `dark`.
    pub theme: String,
    /// Auto-start on login.
    pub auto_start: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_url: "https://drop.example.com".to_string(),
            download_dir: "~/Games/Heretek-Drop".to_string(),
            max_concurrent: 2,
            theme: "auto".to_string(),
            auto_start: false,
        }
    }
}

/// Settings service handle.
#[derive(Clone)]
pub struct SettingsService {
    state: AppState,
    bus: EventBus,
}

impl SettingsService {
    /// Create a new settings service.
    #[must_use]
    pub fn new(state: AppState, bus: EventBus) -> Self {
        Self { state, bus }
    }

    /// Load settings from disk.
    pub async fn load(&self) -> Settings {
        let db = self.state.db().await;
        match db.read::<Settings>("settings").await {
            Ok(Some(s)) => s,
            Ok(None) => Settings::default(),
            Err(e) => {
                tracing::warn!("failed to load settings, using defaults: {e}");
                Settings::default()
            }
        }
    }

    /// Persist settings to disk.
    pub async fn save(&self, settings: &Settings) -> Result<(), crate::AppError> {
        info!("saving settings");
        let db = self.state.db().await;
        db.write("settings", settings).await?;
        self.bus.try_send(Event::SettingsChanged);
        Ok(())
    }
}
