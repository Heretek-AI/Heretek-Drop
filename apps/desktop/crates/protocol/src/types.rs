// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Drop API types — request/response shapes for `/api/v1/client/*`.

use serde::{Deserialize, Serialize};

/// User profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Server-side user ID.
    pub id: String,
    /// Username.
    pub username: String,
    /// Email address.
    pub email: String,
    /// Role hint (display-only, not access control).
    pub role: String,
    /// Client ID (UUID) for this client.
    pub client_id: String,
}

/// Game summary (for library list).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Server-side game ID.
    pub id: u32,
    /// Game title.
    pub title: String,
    /// Short description.
    pub description: String,
    /// URL to the cover image.
    pub cover_url: String,
    /// URL to the banner image.
    pub banner_url: String,
    /// Developer names.
    pub developers: Vec<String>,
    /// Publisher names.
    pub publishers: Vec<String>,
    /// Genre tags.
    pub genres: Vec<String>,
    /// Supported platforms (e.g. "linux", "windows", "macos").
    pub platforms: Vec<String>,
    /// PEGI/ESRB age rating.
    pub age_rating: Option<String>,
    /// ISO 8601 release date.
    pub release_date: Option<String>,
    /// Average playtime in minutes.
    pub average_playtime: Option<u32>,
}

/// Game version download option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloadOption {
    /// Version ID.
    pub id: u32,
    /// Version string (e.g. "1.0.0").
    pub version: String,
    /// Download size in bytes.
    pub size_bytes: u64,
    /// Direct download URL.
    pub download_url: String,
    /// SHA-256 checksum, hex-encoded.
    pub checksum: String,
    /// Target platform (e.g. "linux").
    pub platform: String,
}

/// Game ID (newtype).
pub type GameId = u32;
