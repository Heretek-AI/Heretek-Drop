// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Credentials: ES384 private key + certificate + client ID.
//!
//! Stored on disk at `~/.config/heretek-drop/credentials.json` with `0600` mode.
//! Private key is the base64-encoded PKCS#8 DER bytes.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn};

/// Client credentials obtained from the handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Base64-encoded ES384 private key (PKCS#8 DER).
    pub private: String,
    /// Base64-encoded ES384 certificate (X.509 DER).
    pub certificate: String,
    /// Client ID issued by the server.
    pub id: String,
}

/// Errors during credential persistence.
#[derive(Debug, Error)]
pub enum CredentialsError {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Could not determine config directory.
    #[error("no config directory available")]
    NoConfigDir,
}

/// Disk-backed credentials storage.
pub struct CredentialsStorage;

impl CredentialsStorage {
    /// Default location: `~/.config/heretek-drop/credentials.json`.
    pub fn default_path() -> Result<PathBuf, CredentialsError> {
        let dirs = directories::ProjectDirs::from("dev", "heretek", "drop")
            .ok_or(CredentialsError::NoConfigDir)?;
        Ok(dirs.config_dir().join("credentials.json"))
    }

    /// Load credentials from the default location.
    /// Returns `Ok(None)` if no credentials file exists.
    pub fn load() -> Result<Option<Credentials>, CredentialsError> {
        let path = Self::default_path()?;
        if !path.exists() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(&path)?;
        let creds: Credentials = serde_json::from_str(&text)?;
        Ok(Some(creds))
    }

    /// Save credentials to the default location with `0600` permissions.
    pub fn save(creds: &Credentials) -> Result<(), CredentialsError> {
        let path = Self::default_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(creds)?;
        std::fs::write(&path, json)?;

        // Set 0600 permissions on Unix. Fail silently on other platforms.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&path) {
                let mut perms = metadata.permissions();
                perms.set_mode(0o600);
                if let Err(e) = std::fs::set_permissions(&path, perms) {
                    warn!("failed to set 0600 permissions on credentials file: {e}");
                }
            }
        }

        info!("credentials saved to {}", path.display());
        Ok(())
    }

    /// Delete credentials from the default location.
    /// Returns `Ok(())` if the file did not exist.
    pub fn clear() -> Result<(), CredentialsError> {
        let path = Self::default_path()?;
        if path.exists() {
            std::fs::remove_file(&path)?;
            info!("credentials cleared from {}", path.display());
        }
        Ok(())
    }
}

/// Convenience: load credentials from the default location.
impl Credentials {
    /// Load credentials from the default location. Returns `None` if absent.
    pub fn load_from_default_location() -> Result<Option<Self>, CredentialsError> {
        CredentialsStorage::load()
    }

    /// Save these credentials to the default location.
    pub fn save_to_default_location(&self) -> Result<(), CredentialsError> {
        CredentialsStorage::save(self)
    }
}
