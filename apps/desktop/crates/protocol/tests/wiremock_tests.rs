// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Wiremock-based HTTP tests for the Drop protocol client.

use heretek_drop_protocol::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Test healthcheck endpoint returns server info.
#[tokio::test]
async fn health_returns_app_name() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "app_name": "Drop",
            "version": "0.1.0"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::new(&mock_server.uri());
    let res = client.health().await.unwrap();
    assert_eq!(res.app_name, "Drop");
    assert_eq!(res.version.unwrap(), "0.1.0");
}

/// Test `auth_initiate` returns a redirect URL.
#[tokio::test]
async fn auth_initiate_ok() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/client/auth/initiate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "client-abc",
            "redirect_url": "https://auth.drop.test/callback?code=xyz",
            "expires_at": "2026-12-31T23:59:59Z"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::new(&mock_server.uri());
    let res = client.auth_initiate().await.unwrap();
    assert_eq!(res.id, "client-abc");
    assert!(res.redirect_url.contains("drop.test"));
}

/// Test `auth_handshake` returns credentials.
#[tokio::test]
async fn auth_handshake_ok() {
    let mock_server = MockServer::start().await;
    // Ensure the body contains {"code":"test-code"}
    let body_check = serde_json::json!({"code": "test-code"});
    Mock::given(method("POST"))
        .and(path("/api/v1/client/auth/handshake"))
        .and(wiremock::matchers::body_partial_json(body_check))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "private": "cHJpdmF0ZS1rZXktYnNlNjQ=",
            "certificate": "Y2VydGlmaWNhdGUtYnNlNjQ=",
            "id": "client-xyz"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::new(&mock_server.uri());
    let res = client.auth_handshake("test-code").await.unwrap();
    assert_eq!(res.id, "client-xyz");
    assert!(res.private.contains("cHJpdmF0"));
}

/// Test server error returns `ProtocolError::Server`.
#[tokio::test]
async fn server_error_returns_protocol_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = Client::new(&mock_server.uri());
    let err = client.health().await.unwrap_err();
    match err {
        heretek_drop_protocol::ProtocolError::Server { status, .. } => {
            assert_eq!(status, 500);
        }
        other => panic!("expected Server error, got {other:?}"),
    }
}

/// Test 401 error returns `ProtocolError::Server`.
#[tokio::test]
async fn unauthorized_returns_protocol_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/client/auth/handshake"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "statusCode": 401,
            "statusMessage": "Unauthorized",
            "message": "Invalid JWT signature"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::new(&mock_server.uri());
    let err = client.auth_handshake("bad-code").await.unwrap_err();
    match err {
        heretek_drop_protocol::ProtocolError::Server { status, .. } => {
            assert_eq!(status, 401);
        }
        other => panic!("expected Server error, got {other:?}"),
    }
}

/// Test `auth_code_request` and `auth_code_poll` URL building.
#[test]
fn endpoint_url_matches_expected() {
    let eps = heretek_drop_protocol::Endpoints::new("https://drop.test").unwrap();
    assert_eq!(
        eps.auth_code_request().as_str(),
        "https://drop.test/api/v1/client/auth/code"
    );
    assert_eq!(
        eps.auth_code_poll("xyz789").as_str(),
        "https://drop.test/api/v1/client/auth/code/xyz789"
    );
}
