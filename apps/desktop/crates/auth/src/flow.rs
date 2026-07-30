// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Auth flow orchestration.

use std::time::Duration;

use thiserror::Error;
use tracing::{info, warn};

use heretek_drop_protocol::Client;

use crate::credentials::Credentials;

/// Auth flow error variants.
#[derive(Debug, Error)]
pub enum AuthError {
    /// HTTP / protocol error.
    #[error("protocol error: {0}")]
    Protocol(#[from] heretek_drop_protocol::ProtocolError),

    /// Timed out waiting for user to complete browser auth.
    #[error("auth flow timed out after {0:?}")]
    Timeout(Duration),

    /// User denied access or browser flow failed.
    #[error("auth denied: {0}")]
    Denied(String),

    /// Other error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type for auth operations.
pub type Result<T> = std::result::Result<T, AuthError>;

/// Auth flow handle. Holds the protocol client and latest session state.
#[derive(Debug, Clone)]
pub struct AuthFlow {
    client: Client,
    state: AuthState,
}

/// Internal state of the auth flow.
#[derive(Debug, Clone, Default)]
pub struct AuthState {
    /// Issued client ID, set after `initiate`.
    pub client_id: Option<String>,
    /// Auth code, set after `start_code_flow`.
    pub auth_code: Option<String>,
}

impl AuthFlow {
    /// Construct a new auth flow on the given protocol client.
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            state: AuthState::default(),
        }
    }

    /// Get current auth state.
    #[must_use]
    pub fn state(&self) -> &AuthState {
        &self.state
    }

    /// Step 1: Initiate the auth flow. Returns the redirect URL to open in the browser.
    pub async fn initiate(&mut self) -> Result<String> {
        info!("initiating auth flow");
        let res = self.client.auth_initiate().await?;
        self.state.client_id = Some(res.id.clone());
        info!(client_id = %res.id, "auth flow initiated");
        Ok(res.redirect_url)
    }

    /// Step 2: Wait for the user to complete the browser flow, then handshake.
    /// Currently a stub — calls `auth_handshake` with the most recent code.
    pub async fn complete_with_code(&mut self, code: &str) -> Result<Credentials> {
        info!("completing auth handshake with code");
        let handshake = self.client.auth_handshake(code).await?;
        let creds = Credentials {
            private: handshake.private,
            certificate: handshake.certificate,
            id: handshake.id,
        };
        creds
            .save_to_default_location()
            .map_err(|e| AuthError::Other(e.into()))?;
        info!(client_id = %creds.id, "auth complete and credentials saved");
        Ok(creds)
    }

    /// Sign out: clear credentials from disk.
    pub fn sign_out(&self) -> Result<()> {
        warn!("signing out and clearing credentials");
        crate::credentials::CredentialsStorage::clear().map_err(|e| AuthError::Other(e.into()))?;
        Ok(())
    }

    /// Open the redirect URL in the system browser.
    /// Returns the URL so the caller can spawn a UI-side action.
    #[must_use]
    pub fn browser_url_for_initiation(&self) -> Option<&str> {
        // Caller invokes `rfd`-style or `open` crate separately.
        None
    }
}

/// Long-poll for the auth code with timeout.
///
/// **Stub for v0.1** — the long-poll endpoint URL wiring and the
/// `http_get_json` helper were removed when protocol was refactored.
/// Reimplement in v0.2 using `Client::auth_code_poll(code)` (TODO: add
/// that method to `heretek_drop_protocol::Client`).
#[allow(dead_code)]
pub async fn poll_for_auth_code(
    _client: &Client,
    _code: &str,
    _timeout: Duration,
) -> Result<String> {
    Err(AuthError::Other(anyhow::anyhow!(
        "poll_for_auth_code: not yet implemented for v0.1"
    )))
}
