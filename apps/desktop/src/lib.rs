// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Heretek-Drop library — application lifecycle, state, and glue.

mod app;
mod config;
mod state;

pub use app::App;
pub use config::Config;
pub use state::AppState;
