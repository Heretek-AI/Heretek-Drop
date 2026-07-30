// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Unit tests for the download manager crate.

use heretek_drop_download_manager::{
    DownloadManager, DownloadProgress, DownloadRequest, DownloadState, default_downloads_dir,
};

/// Default downloads dir resolves to a path.
#[test]
fn default_dir_returns_path() {
    let dir = default_downloads_dir();
    assert!(!dir.as_os_str().is_empty());
}

/// DownloadProgress fraction is within [0, 1].
#[test]
fn download_progress_fraction() {
    let p = DownloadProgress {
        id: 1,
        downloaded_bytes: 50,
        total_bytes: Some(100),
        state: DownloadState::Downloading,
        error: None,
    };
    assert!((p.fraction() - 0.5).abs() < f32::EPSILON);
}

/// DownloadProgress fraction returns 0 when total_bytes is 0.
#[test]
fn download_progress_fraction_zero_total() {
    let p = DownloadProgress {
        id: 2,
        downloaded_bytes: 100,
        total_bytes: None,
        state: DownloadState::Downloading,
        error: None,
    };
    assert_eq!(p.fraction(), 0.0);
}

/// DownloadProgress fraction returns 1 when downloaded equals total.
#[test]
fn download_progress_fraction_complete() {
    let p = DownloadProgress {
        id: 3,
        downloaded_bytes: 200,
        total_bytes: Some(200),
        state: DownloadState::Completed,
        error: None,
    };
    assert!((p.fraction() - 1.0).abs() < f32::EPSILON);
}

/// DownloadState derives Debug, Clone, PartialEq.
#[test]
fn download_state_debug_clone() {
    let states = vec![
        DownloadState::Pending,
        DownloadState::Downloading,
        DownloadState::Paused,
        DownloadState::Completed,
        DownloadState::Failed,
        DownloadState::Cancelled,
    ];
    for s in &states {
        let _cloned = s.clone();
        let _debug = format!("{s:?}");
    }
}

/// Enqueue and cancel a download.
#[tokio::test]
async fn enqueue_and_cancel() {
    let dm = DownloadManager::new(2, default_downloads_dir());
    let req = DownloadRequest {
        id: 42,
        title: "Test Game".into(),
        url: "https://httpbin.org/bytes/100".into(),
        checksum: "abc".into(),
        size_bytes: 100,
        dest: std::path::PathBuf::from("/tmp/test-download.bin"),
    };
    let id = dm.enqueue(req).await;
    assert_eq!(id, 42);

    // Cancel should succeed immediately
    dm.cancel(id).await.unwrap();
    assert_eq!(dm.active_count().await, 0);
}

/// Cancelling a non-existent download returns NotFound error.
#[tokio::test]
async fn cancel_nonexistent_returns_error() {
    let dm = DownloadManager::new(2, default_downloads_dir());
    let err = dm.cancel(9999).await.unwrap_err();
    match err {
        heretek_drop_download_manager::DownloadError::NotFound(id) => assert_eq!(id, 9999),
        other => panic!("expected NotFound, got {other:?}"),
    }
}

/// Default capacity is 2 concurrent downloads.
#[test]
fn default_max_concurrent() {
    let dm = DownloadManager::new(2, default_downloads_dir());
    assert_eq!(dm.max_concurrent(), 2);
}
