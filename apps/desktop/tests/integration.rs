// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Integration tests for the main crate.

use heretek_drop::events::{Event, EventBus};
use heretek_drop_database::Database;
use heretek_drop_protocol::Endpoints;

// ── Event Bus ─────────────────────────────────────────────────────────────────

#[test]
fn event_bus_try_send_does_not_block() {
    let bus = EventBus::new(4);
    for i in 0..32 {
        bus.try_send(Event::Toast {
            level: "info".to_string(),
            message: format!("msg {i}"),
        });
    }
    let remaining = bus.receiver().len();
    assert!(remaining <= 4, "should drop excess events, got {remaining}");
}

#[test]
fn event_bus_event_type_name() {
    let e = Event::AuthChanged {
        logged_in: true,
        username: Some("test".into()),
    };
    assert_eq!(e.type_name(), "auth_changed");

    let e = Event::DownloadProgress {
        id: 1,
        downloaded_bytes: 100,
        total_bytes: Some(200),
        state: "downloading".into(),
        error: None,
    };
    assert_eq!(e.type_name(), "download_progress");
}

#[tokio::test]
async fn event_bus_fifo() {
    let bus = EventBus::new(16);
    bus.try_send(Event::Toast {
        level: "info".to_string(),
        message: "first".into(),
    });
    bus.try_send(Event::Toast {
        level: "error".to_string(),
        message: "second".into(),
    });

    let rx = bus.receiver();
    let e1 = rx.recv_async().await.unwrap();
    let e2 = rx.recv_async().await.unwrap();
    assert_eq!(e1.type_name(), "toast");
    assert_eq!(e2.type_name(), "toast");
}

#[tokio::test]
async fn event_bus_drops_when_full() {
    let bus = EventBus::new(1);
    bus.try_send(Event::Toast {
        level: "info".to_string(),
        message: "first".into(),
    });
    bus.try_send(Event::Toast {
        level: "error".to_string(),
        message: "dropped".into(),
    });
    let rx = bus.receiver();
    let _first = rx.recv_async().await.unwrap();
    let second = tokio::time::timeout(std::time::Duration::from_millis(100), rx.recv_async()).await;
    assert!(second.is_err(), "second event should have been dropped");
}

// ── Endpoints ─────────────────────────────────────────────────────────────────

#[test]
fn endpoints_url_building() {
    let eps = Endpoints::new("https://drop.test").unwrap();
    assert_eq!(eps.health().as_str(), "https://drop.test/api/v1/");
    assert_eq!(
        eps.auth_initiate().as_str(),
        "https://drop.test/api/v1/client/auth/initiate"
    );
    assert_eq!(
        eps.auth_handshake().as_str(),
        "https://drop.test/api/v1/client/auth/handshake"
    );
    assert_eq!(eps.user().as_str(), "https://drop.test/api/v1/client/user");
    assert_eq!(
        eps.user_library().as_str(),
        "https://drop.test/api/v1/client/user/library"
    );
    assert_eq!(
        eps.game(42).as_str(),
        "https://drop.test/api/v1/client/game/42"
    );
    assert_eq!(
        eps.game_versions(7).as_str(),
        "https://drop.test/api/v1/client/game/7/versions"
    );
}

#[test]
fn endpoints_invalid_base_url() {
    let result = Endpoints::new("not-a-url");
    assert!(result.is_err());
}

// ── Database Integration ─────────────────────────────────────────────────────

#[tokio::test]
async fn database_in_memory_all_types() {
    let db = Database::open_memory().unwrap();
    db.write("str", &"hello".to_string()).await.unwrap();
    db.write("int", &42_i32).await.unwrap();
    db.write("float", &3.14_f64).await.unwrap();
    db.write("bool", &true).await.unwrap();

    assert_eq!(
        db.read::<String>("str").await.unwrap(),
        Some("hello".into())
    );
    assert_eq!(db.read::<i32>("int").await.unwrap(), Some(42));
    assert_eq!(db.read::<f64>("float").await.unwrap(), Some(3.14));
    assert_eq!(db.read::<bool>("bool").await.unwrap(), Some(true));
}

#[tokio::test]
async fn database_keys_isolated() {
    let db = Database::open_memory().unwrap();
    db.write("a", &1_i32).await.unwrap();
    db.write("b", &2_i32).await.unwrap();

    assert_eq!(db.read::<i32>("a").await.unwrap(), Some(1));
    assert_eq!(db.read::<i32>("b").await.unwrap(), Some(2));

    db.delete("a").await.unwrap();
    assert_eq!(db.read::<i32>("a").await.unwrap(), None);
    assert_eq!(db.read::<i32>("b").await.unwrap(), Some(2));
}
