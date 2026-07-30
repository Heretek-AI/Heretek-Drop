// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Application configuration loaded from `~/.config/heretek-drop/config.toml`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Top-level config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Drop server config.
    pub server: ServerConfig,
    /// UI config.
    #[serde(default)]
    pub ui: UiConfig,
    /// Downloads config.
    #[serde(default)]
    pub downloads: DownloadsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Base URL of the Drop server (e.g. `https://drop.example.com`).
    pub base_url: String,
    /// Optional timeout in seconds. Default: 30.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme: `light`, `dark`, or `auto`. Default: `auto`.
    #[serde(default = "default_theme")]
    pub theme: String,
    /// Window start width. Default: 1280.
    #[serde(default = "default_width")]
    pub window_width: u32,
    /// Window start height. Default: 800.
    #[serde(default = "default_height")]
    pub window_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadsConfig {
    /// Directory to download games to. Default: `~/Games`.
    #[serde(default)]
    pub directory: Option<PathBuf>,
    /// Max concurrent downloads. Default: 2.
    #[serde(default = "default_concurrent_downloads")]
    pub max_concurrent: u32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            window_width: default_width(),
            window_height: default_height(),
        }
    }
}

impl Default for DownloadsConfig {
    fn default() -> Self {
        Self {
            directory: None,
            max_concurrent: default_concurrent_downloads(),
        }
    }
}

fn default_timeout() -> u64 {
    30
}
fn default_theme() -> String {
    "auto".to_string()
}
fn default_width() -> u32 {
    1280
}
fn default_height() -> u32 {
    800
}
fn default_concurrent_downloads() -> u32 {
    2
}

impl Config {
    /// Load config from the default location, falling back to defaults if missing.
    pub fn load_or_default() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            info!("config not found at {}, using defaults", path.display());
            return Ok(Self::default_config());
        }
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("read config from {}", path.display()))?;
        toml::from_str(&text).with_context(|| format!("parse config at {}", path.display()))
    }

    /// Default config — points at the official Drop instance.
    #[must_use]
    pub fn default_config() -> Self {
        Self {
            server: ServerConfig {
                base_url: "https://drop.example.com".to_string(),
                timeout_secs: default_timeout(),
            },
            ui: UiConfig::default(),
            downloads: DownloadsConfig::default(),
        }
    }
}

/// Default config path: `~/.config/heretek-drop/config.toml`.
pub fn config_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "heretek", "drop")
        .context("no config directory available")?;
    Ok(dirs.config_dir().join("config.toml"))
}
