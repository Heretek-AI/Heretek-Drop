// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Drop REST API client.
//!
//! Lightweight HTTP client over `reqwest` that:
//! - Prepends `{base_url}/api/v1/client/` to all paths
//! - Injects the JWT auth header on every request
//! - Returns typed errors with retry hints
//!
//! Does NOT handle auth flow (use `heretek_drop_auth` for that).

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod client;
mod endpoints;
mod types;

pub use client::{AuthHeader, Client};
pub use types::{Game, GameId, User, VersionDownloadOption};
