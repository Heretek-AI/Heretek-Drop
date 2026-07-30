// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Shared types and errors used across all Heretek-Drop sub-crates.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Top-level library error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("server returned status {status}: {message}")]
    Server { status: u16, message: String },

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Returns `true` if the error indicates a transient server-side failure.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Network(_) => true,
            Error::Server { status, .. } => *status >= 500,
            Error::Io(_) => true,
            _ => false,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e.to_string())
    }
}

/// Result alias using our error type.
pub type Result<T> = std::result::Result<T, Error>;

/// A typed ID wrapper to avoid mixing different ID types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(pub u32);

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for GameId {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

/// Drop server base URL.
#[derive(Debug, Clone)]
pub struct ServerUrl(pub url::Url);

impl ServerUrl {
    /// Parse from string, requiring HTTPS (or HTTP for localhost dev).
    pub fn parse(s: &str) -> Result<Self> {
        let url =
            url::Url::parse(s).map_err(|e| Error::InvalidInput(format!("invalid URL: {e}")))?;
        let scheme = url.scheme();
        if scheme != "https" && scheme != "http" {
            return Err(Error::InvalidInput(format!(
                "URL must use http(s), got {scheme}"
            )));
        }
        Ok(Self(url))
    }

    /// Get the URL as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for ServerUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
