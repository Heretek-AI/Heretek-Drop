// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Drop auth flow.
//!
//! Handles:
//! - Browser-initiated deep-link flow
//! - Code-based flow with WebSocket polling
//! - Storing credentials in the OS keyring (private key) + JSON (id + token)
//!
//! Does NOT perform the actual HTTP requests (use `heretek_drop_protocol`).

#![forbid(unsafe_code)]

mod credentials;
mod error;
mod flow;

pub use credentials::{Credentials, CredentialsStorage};
pub use error::{AuthError, Result};
pub use flow::AuthFlow;
