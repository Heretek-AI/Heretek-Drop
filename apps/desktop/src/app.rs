// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Application lifecycle: load config, build state, run UI, drain events.

use anyhow::{Context, Result};
use slint::Model;
use tracing::{error, info};

use crate::state::AppState;

/// Top-level app handle.
pub struct App {
    state: AppState,
    ui: crate::MainWindow,
}

impl App {
    /// Initialize app: load config, build state, build UI.
    pub fn new() -> Result<Self> {
        let config = crate::config::Config::load_or_default().context("load config")?;
        let state = AppState::new(config).context("build app state")?;
        let ui = crate::MainWindow::new().context("build main window")?;

        Ok(Self { state, ui })
    }

    /// Run the app: wire UI callbacks, show window, block until exit.
    pub fn run(self) -> Result<()> {
        let App { state, ui } = self;
        ui.set_app_name("Heretek-Drop".into());
        ui.set_window_title("Heretek-Drop".into());

        // Wire callbacks here. Stub for now — Wave 2 will fill in.

        ui.run().context("UI run loop exited with error")?;
        info!("UI exited cleanly");
        Ok(())
    }
}
