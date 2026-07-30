// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Database error types.

use thiserror::Error;

/// Result alias for database operations.
pub type Result<T> = std::result::Result<T, DbError>;

/// Database error variants.
#[derive(Debug, Error)]
pub enum DbError {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Store corruption.
    #[error("store corruption: {0}")]
    Corruption(String),

    /// Config directory unavailable.
    #[error("no config directory available")]
    NoConfigDir,
}
