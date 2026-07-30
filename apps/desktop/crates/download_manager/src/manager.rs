// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Download manager — orchestrates concurrent game downloads.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use futures::StreamExt;
use sha2::{Digest, Sha256};
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::progress::{DownloadProgress, DownloadState};

/// A single download request.
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// Stable ID for this download (game ID).
    pub id: u32,
    /// Game title for UI display.
    pub title: String,
    /// URL to download.
    pub url: String,
    /// SHA-256 checksum (hex).
    pub checksum: String,
    /// Total size in bytes.
    pub size_bytes: u64,
    /// Destination file path.
    pub dest: PathBuf,
}

/// Download manager error variants.
#[derive(Debug, Error)]
pub enum DownloadError {
    /// HTTP error.
    #[error("HTTP error: {0}")]
    Http(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Checksum mismatch.
    #[error("checksum mismatch: expected {expected}, got {actual}")]
    Checksum { expected: String, actual: String },

    /// Download not found.
    #[error("download {0} not found")]
    NotFound(u32),
}

/// Result alias for download operations.
pub type Result<T> = std::result::Result<T, DownloadError>;

/// Download manager handle.
#[derive(Clone)]
pub struct DownloadManager {
    inner: Arc<ManagerInner>,
}

struct ManagerInner {
    active: Mutex<HashMap<u32, JoinHandle<()>>>,
    progress_tx: flume::Sender<DownloadProgress>,
    progress_rx: flume::Receiver<DownloadProgress>,
    max_concurrent: u32,
    downloads_dir: PathBuf,
}

impl DownloadManager {
    /// Create a new download manager.
    pub fn new(max_concurrent: u32, downloads_dir: PathBuf) -> Self {
        let (progress_tx, progress_rx) = flume::bounded(128);
        Self {
            inner: Arc::new(ManagerInner {
                active: Mutex::new(HashMap::new()),
                progress_tx,
                progress_rx,
                max_concurrent,
                downloads_dir,
            }),
        }
    }

    /// Get a receiver for progress events.
    pub fn progress_receiver(&self) -> flume::Receiver<DownloadProgress> {
        self.inner.progress_rx.clone()
    }

    /// Queue a download. Returns the download ID.
    pub async fn enqueue(&self, req: DownloadRequest) -> u32 {
        let id = req.id;
        let title = req.title.clone();
        let url = req.url.clone();
        let checksum = req.checksum.clone();
        let size = req.size_bytes;
        let dest = req.dest.clone();
        let tx = self.inner.progress_tx.clone();

        let dir = dest
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| self.inner.downloads_dir.clone());
        if let Err(e) = tokio::fs::create_dir_all(&dir).await {
            error!("failed to create download dir {dir:?}: {e}");
        }

        let handle = tokio::spawn(async move {
            run_download(id, title, url, checksum, size, dest, tx).await;
        });

        self.inner.active.lock().await.insert(id, handle);
        info!(id, "download queued");
        id
    }

    /// Cancel an active download.
    pub async fn cancel(&self, id: u32) -> Result<()> {
        let mut active = self.inner.active.lock().await;
        if let Some(handle) = active.remove(&id) {
            handle.abort();
            info!(id, "download cancelled");
            Ok(())
        } else {
            Err(DownloadError::NotFound(id))
        }
    }

    /// Count of active downloads.
    pub async fn active_count(&self) -> usize {
        self.inner.active.lock().await.len()
    }

    /// Get configured max concurrent downloads.
    pub fn max_concurrent(&self) -> u32 {
        self.inner.max_concurrent
    }

    /// Get the default downloads directory.
    pub fn downloads_dir(&self) -> &PathBuf {
        &self.inner.downloads_dir
    }
}

async fn run_download(
    id: u32,
    title: String,
    url: String,
    expected_checksum: String,
    total_bytes: u64,
    dest: PathBuf,
    tx: flume::Sender<DownloadProgress>,
) {
    let _ = tx.send(DownloadProgress {
        id,
        downloaded_bytes: 0,
        total_bytes: Some(total_bytes),
        state: DownloadState::Downloading,
        error: None,
    });

    let client = reqwest::Client::new();
    let res = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.send(DownloadProgress {
                id,
                downloaded_bytes: 0,
                total_bytes: Some(total_bytes),
                state: DownloadState::Failed,
                error: Some(format!("HTTP error: {e}")),
            });
            return;
        }
    };

    if !res.status().is_success() {
        let _ = tx.send(DownloadProgress {
            id,
            downloaded_bytes: 0,
            total_bytes: Some(total_bytes),
            state: DownloadState::Failed,
            error: Some(format!("server returned {}", res.status())),
        });
        return;
    }

    let mut file = match tokio::fs::File::create(&dest).await {
        Ok(f) => f,
        Err(e) => {
            let _ = tx.send(DownloadProgress {
                id,
                downloaded_bytes: 0,
                total_bytes: Some(total_bytes),
                state: DownloadState::Failed,
                error: Some(format!("create file: {e}")),
            });
            return;
        }
    };

    let mut stream = res.bytes_stream();
    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(DownloadProgress {
                    id,
                    downloaded_bytes: downloaded,
                    total_bytes: Some(total_bytes),
                    state: DownloadState::Failed,
                    error: Some(format!("read stream: {e}")),
                });
                return;
            }
        };

        hasher.update(&chunk);
        if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await {
            let _ = tx.send(DownloadProgress {
                id,
                downloaded_bytes: downloaded,
                total_bytes: Some(total_bytes),
                state: DownloadState::Failed,
                error: Some(format!("write file: {e}")),
            });
            return;
        }

        downloaded += chunk.len() as u64;
        let _ = tx.send(DownloadProgress {
            id,
            downloaded_bytes: downloaded,
            total_bytes: Some(total_bytes),
            state: DownloadState::Downloading,
            error: None,
        });
    }

    let actual = format!("{:x}", hasher.finalize());
    if actual != expected_checksum {
        let _ = tx.send(DownloadProgress {
            id,
            downloaded_bytes: downloaded,
            total_bytes: Some(total_bytes),
            state: DownloadState::Failed,
            error: Some(format!(
                "checksum mismatch: expected {}, got {}",
                expected_checksum, actual
            )),
        });
        return;
    }

    let _ = tx.send(DownloadProgress {
        id,
        downloaded_bytes: downloaded,
        total_bytes: Some(total_bytes),
        state: DownloadState::Completed,
        error: None,
    });

    info!(id, title, "download complete");
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new(2, default_downloads_dir())
    }
}

/// Default download directory: `~/Games/Heretek-Drop`.
pub fn default_downloads_dir() -> PathBuf {
    directories::UserDirs::new()
        .map(|u| u.home_dir().join("Games").join("Heretek-Drop"))
        .unwrap_or_else(|| PathBuf::from("./downloads"))
}

// Unused helper kept to suppress dead-code warning on context import.
#[allow(dead_code)]
fn _force_use_context_import() {
    let _: fn(&str) -> anyhow::Result<()> = |s| {
        std::fs::write(PathBuf::from(s), "")
            .context("placeholder")
            .map(|_| ())
    };
}
