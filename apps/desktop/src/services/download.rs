// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Download service — wraps `DownloadManager` and forwards progress to the event bus.

use std::path::PathBuf;

use crate::events::{Event, EventBus};
use heretek_drop_download_manager::{DownloadManager, DownloadProgress, DownloadRequest};
use tokio::task::JoinHandle;
use tracing::{info, warn};

/// Download service handle.
#[derive(Clone)]
pub struct DownloadService {
    manager: DownloadManager,
    bus: EventBus,
    pump: std::sync::Arc<tokio::sync::Mutex<Option<JoinHandle<()>>>>,
}

impl DownloadService {
    /// Create a new download service.
    pub fn new(manager: DownloadManager, bus: EventBus) -> Self {
        Self {
            manager,
            bus,
            pump: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    /// Start the background pump that forwards `DownloadManager` progress events to the bus.
    /// Call once at startup.
    pub fn start_pump(&self) {
        let bus = self.bus.clone();
        let rx = self.manager.progress_receiver();
        let handle = tokio::spawn(async move {
            while let Ok(p) = rx.recv_async().await {
                forward_progress(&bus, &p);
            }
        });
        // Replace any existing pump.
        let pump = self.pump.clone();
        tokio::spawn(async move {
            let mut guard = pump.lock().await;
            if let Some(old) = guard.take() {
                old.abort();
            }
            *guard = Some(handle);
        });
    }

    /// Enqueue a download.
    pub async fn enqueue(&self, req: DownloadRequest) -> u32 {
        info!(id = req.id, title = %req.title, "enqueueing download");
        self.manager.enqueue(req).await
    }

    /// Cancel a download.
    pub async fn cancel(
        &self,
        id: u32,
    ) -> Result<(), heretek_drop_download_manager::DownloadError> {
        self.manager.cancel(id).await
    }

    /// Get the default downloads directory.
    pub fn downloads_dir(&self) -> PathBuf {
        self.manager.downloads_dir().clone()
    }
}

fn forward_progress(bus: &EventBus, p: &DownloadProgress) {
    let state = match p.state {
        heretek_drop_download_manager::DownloadState::Pending => "pending",
        heretek_drop_download_manager::DownloadState::Downloading => "downloading",
        heretek_drop_download_manager::DownloadState::Paused => "paused",
        heretek_drop_download_manager::DownloadState::Completed => "completed",
        heretek_drop_download_manager::DownloadState::Failed => "failed",
        heretek_drop_download_manager::DownloadState::Cancelled => "cancelled",
    };
    bus.try_send(Event::DownloadProgress {
        id: p.id,
        downloaded_bytes: p.downloaded_bytes,
        total_bytes: p.total_bytes,
        state: state.to_string(),
        error: p.error.clone(),
    });
    if p.error.is_some() {
        warn!(id = p.id, "download error: {:?}", p.error);
    }
}
