// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Event bus for decoupled UI ↔ background task communication.

use flume::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Top-level event types emitted by background tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    /// Authentication state changed.
    AuthChanged {
        /// Whether the user is now logged in.
        logged_in: bool,
        /// Username (if logged in).
        username: Option<String>,
    },

    /// Auth flow error.
    AuthError {
        /// Error message to display.
        message: String,
    },

    /// Library loaded.
    LibraryLoaded {
        /// Number of games loaded.
        count: usize,
    },

    /// Library error.
    LibraryError {
        /// Error message.
        message: String,
    },

    /// Download progress update.
    DownloadProgress {
        /// Download ID.
        id: u32,
        /// Downloaded bytes.
        downloaded_bytes: u64,
        /// Total bytes.
        total_bytes: Option<u64>,
        /// State string.
        state: String,
        /// Error (if any).
        error: Option<String>,
    },

    /// Game launched.
    GameLaunched {
        /// Game ID.
        id: u32,
        /// Process ID.
        pid: u32,
    },

    /// Game process exited.
    GameExited {
        /// Game ID.
        id: u32,
        /// Exit code.
        exit_code: i32,
    },

    /// Settings changed.
    SettingsChanged,

    /// Generic toast notification.
    Toast {
        /// Toast level: `info`, `success`, `warning`, `error`.
        level: String,
        /// Message.
        message: String,
    },
}

impl Event {
    /// Get the type name as a string (for log filtering).
    pub fn type_name(&self) -> &'static str {
        match self {
            Event::AuthChanged { .. } => "auth_changed",
            Event::AuthError { .. } => "auth_error",
            Event::LibraryLoaded { .. } => "library_loaded",
            Event::LibraryError { .. } => "library_error",
            Event::DownloadProgress { .. } => "download_progress",
            Event::GameLaunched { .. } => "game_launched",
            Event::GameExited { .. } => "game_exited",
            Event::SettingsChanged => "settings_changed",
            Event::Toast { .. } => "toast",
        }
    }
}

/// Cheap-to-clone event bus.
#[derive(Clone)]
pub struct EventBus {
    inner: Arc<Inner>,
}

struct Inner {
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl EventBus {
    /// Create a new event bus with a bounded channel.
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = flume::bounded(capacity);
        Self {
            inner: Arc::new(Inner { tx, rx }),
        }
    }

    /// Get a sender (cheap to clone).
    pub fn sender(&self) -> Sender<Event> {
        self.inner.tx.clone()
    }

    /// Get a receiver (one per consumer).
    pub fn receiver(&self) -> Receiver<Event> {
        self.inner.rx.clone()
    }

    /// Send an event, dropping if the channel is full (with a log warning).
    pub fn try_send(&self, event: Event) {
        if let Err(e) = self.inner.tx.try_send(event.clone()) {
            tracing::warn!(event = %event.type_name(), "event bus full, dropping event: {e}");
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}
