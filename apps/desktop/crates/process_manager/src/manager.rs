// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Process manager — spawns game processes and tracks running PIDs.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::process::Command;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::process::{GameProcess, LaunchSpec, ProcessError};

/// Process manager handle.
#[derive(Clone)]
pub struct ProcessManager {
    inner: Arc<ManagerInner>,
}

struct ManagerInner {
    processes: Mutex<HashMap<u32, GameProcess>>,
}

impl ProcessManager {
    /// Create a new process manager.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ManagerInner {
                processes: Mutex::new(HashMap::new()),
            }),
        }
    }

    /// Spawn a game process.
    pub async fn launch(&self, spec: LaunchSpec) -> Result<GameProcess, ProcessError> {
        if !spec.executable.exists() {
            return Err(ProcessError::NotFound(spec.executable.clone()));
        }

        let workdir = spec
            .working_dir
            .clone()
            .unwrap_or_else(|| {
                spec.executable
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_default()
            });

        let mut cmd = Command::new(&spec.executable);
        cmd.args(&spec.args);
        if !workdir.as_os_str().is_empty() {
            cmd.current_dir(&workdir);
        }
        for (k, v) in &spec.env {
            cmd.env(k, v);
        }

        let child = cmd.spawn()?;
        let pid = child.id().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "no pid assigned to child")
        })?;

        let proc = GameProcess {
            id: spec.id,
            title: spec.title.clone(),
            pid,
            started_at: Utc::now(),
        };

        self.inner.processes.lock().await.insert(spec.id, proc.clone());
        info!(id = spec.id, title = %spec.title, pid, "game launched");
        Ok(proc)
    }

    /// Get info about a tracked game process.
    pub async fn get(&self, id: u32) -> Option<GameProcess> {
        self.inner.processes.lock().await.get(&id).cloned()
    }

    /// List all tracked processes.
    pub async fn list(&self) -> Vec<GameProcess> {
        self.inner.processes.lock().await.values().cloned().collect()
    }

    /// Kill a tracked game process.
    pub async fn kill(&self, id: u32) -> Option<()> {
        let proc = self.inner.processes.lock().await.remove(&id)?;
        warn!(id, pid = proc.pid, "killing game process");
        // Best-effort: signal the PID. Doesn't catch already-exited processes.
        let _ = std::process::Command::new("kill")
            .arg("-TERM")
            .arg(proc.pid.to_string())
            .status();
        Some(())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
