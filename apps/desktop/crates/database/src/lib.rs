// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Local JSON-on-disk key-value store.
//!
//! Used for caching library snapshots, user preferences, and download states.

#![forbid(unsafe_code)]

mod cache;
mod db;
mod error;

pub use cache::{Cache, CachedLibrary};
pub use db::Database;
pub use error::{DbError, Result};
