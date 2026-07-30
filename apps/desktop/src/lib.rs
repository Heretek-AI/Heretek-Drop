// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Heretek-Drop library — application lifecycle, state, glue, and UI commands.

#![deny(unsafe_code)]
#![allow(missing_docs)] // Slint-generated code has no docs

// Generated Slint types from build.rs.
slint::include_modules!();

mod app;
mod commands;
mod config;
mod error;
pub mod events;
mod services;
mod state;

pub use app::App;
pub use commands::Commands;
pub use config::Config;
pub use error::{AppError, Result};
pub use events::{Event, EventBus};
pub use services::{AuthService, DownloadService, LibraryService, SettingsService};
pub use state::AppState;
