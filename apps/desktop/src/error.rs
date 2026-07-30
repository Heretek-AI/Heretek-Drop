// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Top-level application error type.

use thiserror::Error;

/// Top-level app error.
#[derive(Debug, Error)]
pub enum AppError {
    /// Config error.
    #[error("config error: {0}")]
    Config(String),

    /// Auth error.
    #[error("auth error: {0}")]
    Auth(#[from] heretek_drop_auth::AuthError),

    /// Protocol error.
    #[error("protocol error: {0}")]
    Protocol(#[from] heretek_drop_protocol::ProtocolError),

    /// Database error.
    #[error("database error: {0}")]
    Database(#[from] heretek_drop_database::DbError),

    /// IO error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Slint error.
    #[error("Slint error: {0}")]
    Slint(String),

    /// Other error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result alias for app operations.
pub type Result<T> = std::result::Result<T, AppError>;
