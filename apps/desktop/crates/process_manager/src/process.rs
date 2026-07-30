// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Game process type.

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A running game process.
#[derive(Debug, Clone)]
pub struct GameProcess {
    /// Game ID.
    pub id: u32,
    /// Game title.
    pub title: String,
    /// OS process ID.
    pub pid: u32,
    /// When the process was started.
    pub started_at: DateTime<Utc>,
}

/// Launch specification for a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchSpec {
    /// Game ID.
    pub id: u32,
    /// Game title.
    pub title: String,
    /// Executable path.
    pub executable: PathBuf,
    /// Working directory (defaults to executable's parent).
    #[serde(default)]
    pub working_dir: Option<PathBuf>,
    /// Command-line arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

/// Process-spawn error variants.
#[derive(Debug, Error)]
pub enum ProcessError {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Executable not found.
    #[error("executable not found: {0}")]
    NotFound(PathBuf),
}
