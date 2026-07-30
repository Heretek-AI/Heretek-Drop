// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Drop API endpoint URLs.

use url::Url;

/// Endpoint URL builder. Holds the base URL (e.g. `https://drop.example.com`).
#[derive(Debug, Clone)]
pub struct Endpoints {
    base: Url,
}

impl Endpoints {
    /// Build endpoint URLs from a base URL.
    pub fn new(base: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            base: Url::parse(base)?,
        })
    }

    /// `GET /api/v1/`
    #[must_use]
    pub fn health(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/");
        u
    }

    /// `POST /api/v1/client/auth/initiate`
    #[must_use]
    pub fn auth_initiate(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/client/auth/initiate");
        u
    }

    /// `POST /api/v1/client/auth/handshake`
    #[must_use]
    pub fn auth_handshake(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/client/auth/handshake");
        u
    }

    /// `POST /api/v1/client/auth/code`
    #[must_use]
    pub fn auth_code_request(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/client/auth/code");
        u
    }

    /// `GET /api/v1/client/auth/code/{code}`
    #[must_use]
    pub fn auth_code_poll(&self, code: &str) -> Url {
        let mut u = self.base.clone();
        u.set_path(&format!("/api/v1/client/auth/code/{code}"));
        u
    }

    /// `GET /api/v1/client/user`
    #[must_use]
    pub fn user(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/client/user");
        u
    }

    /// `GET /api/v1/client/user/library`
    #[must_use]
    pub fn user_library(&self) -> Url {
        let mut u = self.base.clone();
        u.set_path("/api/v1/client/user/library");
        u
    }

    /// `GET /api/v1/client/game/{id}`
    #[must_use]
    pub fn game(&self, id: u32) -> Url {
        let mut u = self.base.clone();
        u.set_path(&format!("/api/v1/client/game/{id}"));
        u
    }

    /// `GET /api/v1/client/game/{id}/versions`
    #[must_use]
    pub fn game_versions(&self, id: u32) -> Url {
        let mut u = self.base.clone();
        u.set_path(&format!("/api/v1/client/game/{id}/versions"));
        u
    }
}
