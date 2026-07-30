// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! UI commands — bridges Slint callbacks to services.

use std::path::PathBuf;

use heretek_drop_download_manager::DownloadRequest;
use heretek_drop_protocol::Game;
use slint::{ModelRc, VecModel};
use tracing::{error, info, warn};

use crate::AppError;
use crate::services::{AuthService, DownloadService, LibraryService, SettingsService};
use crate::state::AppState;

/// Commands — all Slint callback handlers.
#[derive(Clone)]
pub struct Commands {
    state: AppState,
    auth: AuthService,
    library: LibraryService,
    downloads: DownloadService,
    settings_service: SettingsService,
}

impl Commands {
    /// Create a new commands handle.
    #[must_use]
    pub fn new(
        state: AppState,
        auth: AuthService,
        library: LibraryService,
        downloads: DownloadService,
        settings_service: SettingsService,
    ) -> Self {
        Self {
            state,
            auth,
            library,
            downloads,
            settings_service,
        }
    }

    /// Handle `login` callback from `LoginPage`.
    pub async fn login(&self, _username: String, _password: String) -> Result<(), AppError> {
        info!("login callback");
        let url = self.auth.start_browser_flow().await?;
        self.auth.open_url_in_browser(&url)?;
        Ok(())
    }

    /// Handle `navigate-to` callback.
    pub fn navigate_to(&self, ui: &crate::MainWindow, page: i32) {
        use crate::Page;
        let target = match page {
            0 => Page::Login,
            1 => Page::Library,
            2 => Page::Game,
            3 => Page::Downloads,
            4 => Page::Settings,
            _ => Page::Login,
        };
        ui.set_current_page(target);
    }

    /// Handle `download-game` callback.
    pub async fn download_game(
        &self,
        ui: &crate::MainWindow,
        game_id: i32,
    ) -> Result<(), AppError> {
        info!(game_id, "download_game callback");
        let game_id_u32 = if let Ok(v) = u32::try_from(game_id) {
            v
        } else {
            error!("download_game: invalid game id {game_id}");
            return Ok(());
        };

        // Resolve to a VersionDownloadOption.
        let version = self.library.resolve_download(game_id_u32).await?;
        let title = version.version.clone();

        let dest = self
            .downloads
            .downloads_dir()
            .join(format!("{game_id_u32}.bin"));
        let req = DownloadRequest {
            id: game_id_u32,
            title: title.clone(),
            url: version.download_url,
            checksum: version.checksum,
            size_bytes: version.size_bytes,
            dest,
        };
        self.downloads.enqueue(req).await;
        ui.set_current_page(crate::Page::Downloads);
        Ok(())
    }

    /// Handle `cancel-download` callback.
    pub async fn cancel_download(&self, id: i32) -> Result<(), AppError> {
        if let Ok(id_u32) = u32::try_from(id) {
            if let Err(e) = self.downloads.cancel(id_u32).await {
                warn!("cancel_download: {e}");
            }
        }
        Ok(())
    }

    /// Handle `launch-game` callback.
    pub async fn launch_game(&self, game_id: i32) -> Result<(), AppError> {
        // TODO: wire process_manager in Wave 3.1
        info!(
            game_id,
            "launch_game: stub (process_manager wiring pending)"
        );
        Ok(())
    }

    /// Handle sign-out button.
    pub async fn sign_out(&self, ui: &crate::MainWindow) -> Result<(), AppError> {
        self.auth.sign_out().await?;
        ui.set_current_page(crate::Page::Login);
        ui.set_is_logged_in(false);
        ui.set_username(Default::default());
        Ok(())
    }

    /// Handle theme toggle.
    pub async fn set_theme(&self, ui: &crate::MainWindow, theme: String) -> Result<(), AppError> {
        info!(theme, "set_theme");
        let mut s = self.settings_service.load().await;
        s.theme = theme.clone();
        self.settings_service.save(&s).await?;
        ui.set_theme(theme.into());
        Ok(())
    }

    /// Handle "Browse..." for download directory.
    pub async fn browse_download_dir(&self) -> Result<PathBuf, AppError> {
        // rfd::AsyncFileDialog is async; in Slint context we use the sync dialog from a
        // blocking task. For now, return the current default.
        // TODO: integrate rfd::AsyncFileDialog via spawn_blocking.
        Ok(self.downloads.downloads_dir())
    }

    /// Refresh the library.
    pub async fn refresh_library(&self, ui: &crate::MainWindow) -> Result<(), AppError> {
        let _count = self.library.refresh().await?;
        self.populate_library(ui).await;
        Ok(())
    }

    /// Populate the library page model from the cache.
    pub async fn populate_library(&self, ui: &crate::MainWindow) {
        let lib = self.library.cached().await;
        let games: Vec<crate::Game> = lib
            .map(|l| l.games.into_iter().map(|g| game_to_slint(&g)).collect())
            .unwrap_or_default();
        let model: ModelRc<crate::Game> = ModelRc::new(VecModel::from(games));
        ui.set_library_games(model);
    }

    /// Populate the user info from credentials.
    pub async fn populate_user(&self, ui: &crate::MainWindow) {
        if let Some(creds) = self.state.credentials().await {
            ui.set_is_logged_in(true);
            ui.set_username(creds.id.clone().into());
        } else {
            ui.set_is_logged_in(false);
            ui.set_username(Default::default());
        }
    }
}

fn game_to_slint(g: &Game) -> crate::Game {
    // Cover image is loaded async by the UI; pass an empty image for v0.1.
    // TODO Wave 3.1: load cover via slint::Image::load_from_data.
    crate::Game {
        id: g.id as i32,
        title: g.title.clone().into(),
        developer: g.developers.first().cloned().unwrap_or_default().into(),
        cover_image: slint::Image::default(),
    }
}
