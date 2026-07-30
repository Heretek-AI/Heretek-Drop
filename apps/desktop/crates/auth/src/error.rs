// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Auth error types.

use thiserror::Error;

/// Result alias for auth operations.
pub type Result<T> = std::result::Result<T, AuthError>;

/// Auth error variants.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Keyring access failed.
    #[error("keyring error: {0}")]
    Keyring(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration missing or invalid.
    #[error("config error: {0}")]
    Config(String),

    /// Auth handshake failed.
    #[error("handshake failed: {0}")]
    Handshake(String),

    /// Auth code expired or invalid.
    #[error("auth code expired or invalid")]
    CodeExpired,

    /// User cancelled the flow.
    #[error("user cancelled auth flow")]
    UserCancelled,
}
