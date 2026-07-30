// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Download manager: chunked downloads with progress events.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod manager;
mod progress;

pub use manager::{DownloadManager, DownloadRequest};
pub use progress::{DownloadProgress, DownloadState};
