// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Protocol error types.

use thiserror::Error;

/// Result alias for protocol operations.
pub type Result<T> = std::result::Result<T, ProtocolError>;

/// Protocol error variants.
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// HTTP transport error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JWT signing/parsing error.
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    /// Server returned a non-success status.
    #[error("server returned {status}: {message}")]
    Server { status: u16, message: String },

    /// JSON parsing error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Invalid configuration.
    #[error("invalid config: {0}")]
    Config(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Other / anyhow-style error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
