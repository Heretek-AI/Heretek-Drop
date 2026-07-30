// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Application lifecycle: load config, build state, run UI, drain events.

use anyhow::Context;
use heretek_drop_download_manager::DownloadManager;
use slint::ComponentHandle;
use tracing::{error, info, warn};

use crate::commands::Commands;
use crate::events::EventBus;
use crate::services::{AuthService, DownloadService, LibraryService, SettingsService};
use crate::state::AppState;

/// Top-level app handle.
pub struct App {
    #[allow(dead_code)]
    state: AppState,
    bus: EventBus,
    ui: crate::MainWindow,
    commands: Commands,
}

impl App {
    /// Initialize app: load config, build state, services, UI.
    pub fn new() -> anyhow::Result<Self> {
        let config = crate::config::Config::load_or_default().context("load config")?;
        let state = AppState::new(config).context("build app state")?;
        let bus = EventBus::new(256);

        let downloads_dir = default_downloads_dir();
        let download_manager = DownloadManager::new(2, downloads_dir);
        // Drop the receiver — DownloadService holds its own copy via start_pump.
        let _ = download_manager.progress_receiver();

        let auth = AuthService::new(state.clone(), bus.clone());
        let library = LibraryService::new(state.clone(), bus.clone());
        let downloads = DownloadService::new(download_manager, bus.clone());
        let settings_service = SettingsService::new(state.clone(), bus.clone());
        downloads.start_pump();

        let commands = Commands::new(
            state.clone(),
            auth.clone(),
            library.clone(),
            downloads.clone(),
            settings_service.clone(),
        );

        let ui = crate::MainWindow::new().context("build main window")?;
        ui.set_app_name("Heretek-Drop".into());

        Ok(Self {
            state,
            bus,
            ui,
            commands,
        })
    }

    /// Run the app: wire UI callbacks, start event pump, show window, block until exit.
    pub fn run(self) -> anyhow::Result<()> {
        let App {
            state: _,
            bus,
            ui,
            commands,
        } = self;

        wire_callbacks(&ui, &commands);
        spawn_initial_population(&ui, &commands);
        spawn_event_pump(&ui, bus.receiver());

        info!("UI run loop starting");
        ui.run().context("UI run loop exited with error")?;
        info!("UI exited cleanly");
        Ok(())
    }
}

/// Wire all Slint callbacks to Rust handlers.
fn wire_callbacks(ui: &crate::MainWindow, commands: &Commands) {
    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_login(move |username, password| {
        let cmds = cmds.clone();
        let ui_weak = ui_weak.clone();
        let username = username.to_string();
        let password = password.to_string();
        slint::spawn_local(async move {
            if let Err(e) = cmds.login(username, password).await {
                error!(error = %e, "login failed");
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_login_error(format!("Login failed: {e}").into());
                }
            }
        })
        .expect("spawn_local from UI thread");
    });

    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_navigate_to(move |page| {
        let ui_weak = ui_weak.clone();
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Some(ui) = ui_weak.upgrade() {
                cmds.navigate_to(&ui, page);
            }
        })
        .expect("spawn_local from UI thread");
    });

    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_download_game(move |id| {
        let ui_weak = ui_weak.clone();
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Some(ui) = ui_weak.upgrade() {
                if let Err(e) = cmds.download_game(&ui, id).await {
                    error!(error = %e, "download_game failed");
                }
            }
        })
        .expect("spawn_local from UI thread");
    });

    let cmds = commands.clone();
    ui.on_cancel_download(move |id| {
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Err(e) = cmds.cancel_download(id).await {
                warn!(error = %e, "cancel_download failed");
            }
        })
        .expect("spawn_local from UI thread");
    });

    let cmds = commands.clone();
    ui.on_launch_game(move |id| {
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Err(e) = cmds.launch_game(id).await {
                error!(error = %e, "launch_game failed");
            }
        })
        .expect("spawn_local from UI thread");
    });

    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_sign_out(move || {
        let ui_weak = ui_weak.clone();
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Some(ui) = ui_weak.upgrade() {
                if let Err(e) = cmds.sign_out(&ui).await {
                    error!(error = %e, "sign_out failed");
                }
            }
        })
        .expect("spawn_local from UI thread");
    });

    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_refresh_library(move || {
        let ui_weak = ui_weak.clone();
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            if let Some(ui) = ui_weak.upgrade() {
                if let Err(e) = cmds.refresh_library(&ui).await {
                    error!(error = %e, "refresh_library failed");
                }
            }
        })
        .expect("spawn_local from UI thread");
    });

    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    ui.on_browse_download_dir(move || {
        let ui_weak = ui_weak.clone();
        let cmds = cmds.clone();
        slint::spawn_local(async move {
            match cmds.browse_download_dir().await {
                Ok(path) => {
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_download_dir(path.display().to_string().into());
                    }
                }
                Err(e) => warn!(error = %e, "browse_download_dir failed"),
            }
        })
        .expect("spawn_local from UI thread");
    });
}

/// Spawn an initial population task.
fn spawn_initial_population(ui: &crate::MainWindow, commands: &Commands) {
    let ui_weak = ui.as_weak();
    let cmds = commands.clone();
    slint::spawn_local(async move {
        if let Some(ui) = ui_weak.upgrade() {
            cmds.populate_user(&ui).await;
            cmds.populate_library(&ui).await;
        }
    })
    .expect("spawn_local from UI thread");
}

/// Spawn the event pump on a background thread, dispatching events to the UI thread.
fn spawn_event_pump(ui: &crate::MainWindow, bus_rx: flume::Receiver<crate::Event>) {
    let ui_weak = ui.as_weak();
    std::thread::spawn(move || {
        while let Ok(event) = bus_rx.recv() {
            let ui_weak = ui_weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_weak.upgrade() {
                    apply_event(&ui, event);
                }
            })
            .expect("invoke_from_event_loop");
        }
    });
}

/// Apply an event to the UI state.
fn apply_event(ui: &crate::MainWindow, event: crate::Event) {
    use crate::Event;
    match event {
        Event::AuthChanged {
            logged_in,
            username,
        } => {
            ui.set_is_logged_in(logged_in);
            ui.set_username(username.unwrap_or_default().into());
            ui.set_current_page(if logged_in {
                crate::Page::Library
            } else {
                crate::Page::Login
            });
        }
        Event::AuthError { message } => {
            ui.set_login_error(message.into());
        }
        Event::LibraryLoaded { count } => {
            info!(count, "library loaded event");
            // The UI re-renders on next populate_library call.
        }
        Event::LibraryError { message } => {
            ui.set_login_error(message.into());
        }
        Event::DownloadProgress {
            id,
            downloaded_bytes,
            total_bytes: _,
            state: _,
            error: _,
        } => {
            info!(id, downloaded_bytes, "download progress");
            // TODO Wave 3: update the downloads model via VecModel in App.
            // ModelRc does not expose set_row_data in Slint 1.17 —
            // keep a VecModel<Download> in App and call methods on it.
        }
        Event::GameLaunched { .. } | Event::GameExited { .. } => {
            // No-op for v0.1.
        }
        Event::SettingsChanged => {
            // Settings saved; UI can re-read.
        }
        Event::Toast { level, message } => {
            info!(level, message, "toast");
        }
    }
}

/// Default downloads directory.
pub fn default_downloads_dir() -> std::path::PathBuf {
    heretek_drop_download_manager::default_downloads_dir()
}
