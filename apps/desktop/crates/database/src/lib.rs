// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Local SQLite-backed key-value store using `rustbreak`.
//!
//! Used for caching library snapshots, user preferences, and download states.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod cache;
mod db;
mod error;

pub use cache::{Cache, CachedLibrary};
pub use db::Database;
pub use error::{DbError, Result};
