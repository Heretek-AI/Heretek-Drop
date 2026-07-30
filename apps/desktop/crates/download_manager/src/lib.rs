// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Download manager: chunked downloads with progress events.

#![forbid(unsafe_code)]

mod manager;
mod progress;

pub use manager::{DownloadError, DownloadManager, DownloadRequest, default_downloads_dir};
pub use progress::{DownloadProgress, DownloadState};
