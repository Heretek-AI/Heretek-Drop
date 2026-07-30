// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Download progress events.

use serde::{Deserialize, Serialize};

/// Lifecycle state of a download.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadState {
    /// Queued, not yet started.
    Pending,
    /// Currently downloading.
    Downloading,
    /// Paused by user.
    Paused,
    /// Completed successfully.
    Completed,
    /// Failed with an error.
    Failed,
    /// Cancelled by user.
    Cancelled,
}

/// A single progress event emitted by the download manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    /// Download ID.
    pub id: u32,
    /// Bytes downloaded so far.
    pub downloaded_bytes: u64,
    /// Total bytes (if known).
    pub total_bytes: Option<u64>,
    /// Current state.
    pub state: DownloadState,
    /// Error message (if state is `Failed`).
    pub error: Option<String>,
}

impl DownloadProgress {
    /// Compute progress as a fraction in `[0.0, 1.0]`.
    #[must_use]
    pub fn fraction(&self) -> f32 {
        match self.total_bytes {
            Some(total) if total > 0 => {
                (self.downloaded_bytes as f32 / total as f32).clamp(0.0, 1.0)
            }
            _ => 0.0,
        }
    }
}
