// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Service layer — wraps sub-crate operations and emits events to the bus.

mod auth;
mod download;
mod library;
mod settings;

pub use auth::AuthService;
pub use download::DownloadService;
pub use library::LibraryService;
pub use settings::SettingsService;
