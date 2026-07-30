// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Drop API HTTP client.

use std::time::Duration;

use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, instrument};

use crate::endpoints::Endpoints;
use crate::error::{ProtocolError, Result};
use crate::types::{
    ClientClaims, Game, HandshakeResponse, HealthResponse, InitiateResponse, User,
    VersionDownloadOption,
};

/// Drop API HTTP client.
///
/// Cheap to clone (internal `reqwest::Client` is `Arc`-backed).
#[derive(Debug, Clone)]
pub struct Client {
    http: ReqwestClient,
    endpoints: Endpoints,
    timeout_secs: u64,
}

impl Client {
    /// Build a new client with the default timeout.
    pub fn new(base_url: &str) -> Self {
        Self::builder(base_url).build()
    }

    /// Construct a new client builder.
    pub fn builder(base_url: &str) -> ClientBuilder {
        ClientBuilder::new(base_url)
    }

    /// Set request timeout in seconds.
    pub fn set_timeout_secs(&mut self, secs: u64) {
        self.timeout_secs = secs;
    }

    /// GET `/api/v1/` — healthcheck.
    #[instrument(skip(self))]
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = self.endpoints.health();
        let res = self.http.get(url).send().await?;
        parse_response(res).await
    }

    /// POST `/api/v1/client/auth/initiate` — begin auth flow.
    #[instrument(skip(self))]
    pub async fn auth_initiate(&self) -> Result<InitiateResponse> {
        let url = self.endpoints.auth_initiate();
        let res = self.http.post(url).json(&serde_json::json!({})).send().await?;
        parse_response(res).await
    }

    /// POST `/api/v1/client/auth/handshake` — exchange code for credentials.
    #[instrument(skip(self, code))]
    pub async fn auth_handshake(&self, code: &str) -> Result<HandshakeResponse> {
        let url = self.endpoints.auth_handshake();
        let res = self
            .http
            .post(url)
            .json(&serde_json::json!({ "code": code }))
            .send()
            .await?;
        parse_response(res).await
    }

    /// GET `/api/v1/client/user` — fetch user profile.
    #[instrument(skip(self, credentials))]
    pub async fn user(&self, credentials: &super::super::auth::Credentials) -> Result<User> {
        let url = self.endpoints.user();
        let res = self.http.get(url).headers(auth_headers(credentials)?).send().await?;
        parse_response(res).await
    }

    /// GET `/api/v1/client/user/library` — fetch owned games.
    #[instrument(skip(self, credentials))]
    pub async fn user_library(
        &self,
        credentials: &super::super::auth::Credentials,
    ) -> Result<Vec<Game>> {
        let url = self.endpoints.user_library();
        let res = self
            .http
            .get(url)
            .headers(auth_headers(credentials)?)
            .send()
            .await?;
        parse_response(res).await
    }

    /// GET `/api/v1/client/game/{id}` — fetch game detail.
    #[instrument(skip(self, credentials))]
    pub async fn game(&self, id: u32, credentials: &super::super::auth::Credentials) -> Result<Game> {
        let url = self.endpoints.game(id);
        let res = self.http.get(url).headers(auth_headers(credentials)?).send().await?;
        parse_response(res).await
    }

    /// GET `/api/v1/client/game/{id}/versions` — list download options.
    #[instrument(skip(self, credentials))]
    pub async fn game_versions(
        &self,
        id: u32,
        credentials: &super::super::auth::Credentials,
    ) -> Result<Vec<VersionDownloadOption>> {
        let url = self.endpoints.game_versions(id);
        let res = self.http.get(url).headers(auth_headers(credentials)?).send().await?;
        parse_response(res).await
    }
}

/// Builder for `Client`.
pub struct ClientBuilder {
    base_url: String,
    timeout_secs: u64,
}

impl ClientBuilder {
    fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout_secs: 30,
        }
    }

    /// Set timeout in seconds.
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Consume the builder and produce a `Client`.
    pub fn build(self) -> Client {
        let http = ReqwestClient::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()
            .expect("reqwest client builder requires valid config");
        let endpoints = Endpoints::new(&self.base_url).expect("invalid base URL");
        Client {
            http,
            endpoints,
            timeout_secs: self.timeout_secs,
        }
    }
}

/// Build the `Authorization: JWT <client_id> <token>` header for an authenticated request.
fn auth_headers(
    credentials: &super::super::auth::Credentials,
) -> Result<reqwest::header::HeaderMap> {
    use base64::Engine;
    let private_key_bytes = base64::engine::general_purpose::STANDARD
        .decode(&credentials.private)
        .map_err(|e| ProtocolError::Config(format!("base64 decode private key: {e}")))?;
    let private_key = EncodingKey::from_ec_pem(&private_key_bytes)
        .map_err(|e| ProtocolError::Config(format!("decode private key: {e}")))?;
    let now = Utc::now().timestamp() as u64;
    let claims = ClientClaims {
        nbf: now,
        exp: now + 10,
    };
    let header = Header::new(Algorithm::ES384);
    let token = encode(&header, &claims, &private_key)?;
    let value = format!("JWT {} {}", credentials.id, token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        value.parse().map_err(|e: reqwest::header::InvalidHeaderValue| {
            ProtocolError::Config(format!("invalid auth header: {e}"))
        })?,
    );
    Ok(headers)
}

/// Parse a response into the expected type, mapping non-2xx to `ProtocolError::Server`.
async fn parse_response<T: DeserializeOwned>(res: reqwest::Response) -> Result<T> {
    let status = res.status();
    if !status.is_success() {
        let message = res.text().await.unwrap_or_else(|_| String::new());
        debug!(%status, message, "server returned non-success");
        return Err(ProtocolError::Server {
            status: status.as_u16(),
            message,
        });
    }
    let body = res.json::<T>().await?;
    Ok(body)
}

/// Serialize a body and POST it.
#[allow(dead_code)]
async fn post_json<T: DeserializeOwned, B: Serialize>(
    client: &ReqwestClient,
    url: &reqwest::Url,
    body: &B,
) -> Result<T> {
    let res = client.post(url.clone()).json(body).send().await?;
    parse_response(res).await
}
