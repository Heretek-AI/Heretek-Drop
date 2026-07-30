// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Integration tests for app-level wiring.

#[test]
fn event_bus_try_send_does_not_block() {
    use heretek_drop::events::{Event, EventBus};
    let bus = EventBus::new(8);

    for i in 0..16 {
        bus.try_send(Event::Toast {
            level: "info".to_string(),
            message: format!("msg {i}"),
        });
    }

    // We may drop some due to bounded capacity, but no panic.
    assert!(bus.receiver().len() <= 8);
}

#[test]
fn event_bus_event_type_name() {
    use heretek_drop::events::Event;
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
