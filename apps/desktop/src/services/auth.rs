// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Auth service — orchestrates the Drop auth flow.

use std::sync::Arc;

use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::events::{Event, EventBus};
use crate::state::AppState;
use heretek_drop_auth::AuthFlow;

/// Auth service handle. Cheap to clone.
#[derive(Clone)]
pub struct AuthService {
    state: AppState,
    bus: EventBus,
    flow: Arc<Mutex<Option<AuthFlow>>>,
}

impl AuthService {
    /// Create a new auth service.
    pub fn new(state: AppState, bus: EventBus) -> Self {
        Self {
            state,
            bus,
            flow: Arc::new(Mutex::new(None)),
        }
    }

    /// Start the browser-initiated auth flow.
    ///
    /// 1. Calls `POST /auth/initiate` to get a redirect URL.
    /// 2. Emits a `Toast` event with the URL (the UI opens it in a browser).
    /// 3. Returns the redirect URL for the caller to open.
    pub async fn start_browser_flow(&self) -> Result<String, crate::AppError> {
        info!("starting browser auth flow");
        let api = self.state.api().await;
        let mut flow = AuthFlow::new(api);
        let url = flow
            .initiate()
            .await
            .map_err(|e| crate::AppError::Other(e.into()))?;

        // Stash the flow for the handshake step.
        *self.flow.lock().await = Some(flow);

        // Surface URL via toast (real implementation opens a browser here).
        self.bus.try_send(Event::Toast {
            level: "info".to_string(),
            message: format!("Open this URL in your browser: {url}"),
        });
        Ok(url)
    }

    /// Complete the auth flow with a code (from the browser callback or polling).
    pub async fn complete_with_code(&self, code: &str) -> Result<(), crate::AppError> {
        info!("completing auth with code");
        let mut flow_guard = self.flow.lock().await;
        let flow = flow_guard.as_mut().ok_or_else(|| {
            crate::AppError::Auth(heretek_drop_auth::AuthError::Config(
                "no flow in progress".into(),
            ))
        })?;
        let creds = flow
            .complete_with_code(code)
            .await
            .map_err(|e| crate::AppError::Other(e.into()))?;
        self.state.set_credentials(creds.clone()).await;
        drop(flow_guard);

        self.bus.try_send(Event::AuthChanged {
            logged_in: true,
            username: Some(creds.id.clone()),
        });
        self.bus.try_send(Event::Toast {
            level: "success".to_string(),
            message: "Signed in".to_string(),
        });
        Ok(())
    }

    /// Sign out — clear credentials from disk and state.
    pub async fn sign_out(&self) -> Result<(), crate::AppError> {
        warn!("signing out");
        let api = self.state.api().await;
        let flow = AuthFlow::new(api);
        flow.sign_out()
            .map_err(|e| crate::AppError::Other(e.into()))?;
        *self.flow.lock().await = None;

        self.bus.try_send(Event::AuthChanged {
            logged_in: false,
            username: None,
        });
        self.bus.try_send(Event::Toast {
            level: "info".to_string(),
            message: "Signed out".to_string(),
        });
        Ok(())
    }

    /// Open a URL in the system browser.
    /// Uses `rfd` for portability; on Linux uses `xdg-open` via the `open` crate pattern.
    pub fn open_url_in_browser(&self, url: &str) -> Result<(), crate::AppError> {
        info!(url, "opening in system browser");
        // `open` crate not in deps; use std::process::Command for portability.
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(url)
                .spawn()
                .map_err(crate::AppError::Io)?;
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(url)
                .spawn()
                .map_err(crate::AppError::Io)?;
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("rundll32")
                .arg("url.dll,FileProtocolHandler")
                .arg(url)
                .spawn()
                .map_err(crate::AppError::Io)?;
        }
        Ok(())
    }
}
