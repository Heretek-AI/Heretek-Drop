// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Unit tests for the auth crate.

use tempfile::TempDir;

use heretek_drop_auth::{Credentials, CredentialsStorage};

/// Credentials save and load round-trip.
#[test]
fn credentials_save_and_load() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("credentials.json");

    let creds = Credentials {
        private: "dGVzdC1wcml2YXRlLWtleQ==".into(),
        certificate: "dGVzdC1jZXJ0".into(),
        id: "client-uuid-abc".into(),
    };

    CredentialsStorage::save_to(&creds, &path).unwrap();
    assert!(path.exists());

    // Verify file permissions on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&path).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "credentials file must be 0600");
    }

    let loaded = CredentialsStorage::load_from(&path).unwrap().unwrap();
    assert_eq!(loaded.private, creds.private);
    assert_eq!(loaded.certificate, creds.certificate);
    assert_eq!(loaded.id, creds.id);
}

/// Loading from non-existent file returns Ok(None).
#[test]
fn load_nonexistent_returns_none() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nonexistent.json");
    let result = CredentialsStorage::load_from(&path).unwrap();
    assert!(result.is_none());
}

/// Credentials clear removes the file.
#[test]
fn credentials_clear_removes_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("to-clear.json");

    let creds = Credentials {
        private: "dGVzdA==".into(),
        certificate: "dGVzdA==".into(),
        id: "client-1".into(),
    };
    CredentialsStorage::save_to(&creds, &path).unwrap();
    assert!(path.exists());

    CredentialsStorage::clear_at(&path).unwrap();
    assert!(!path.exists());
}

/// Saving creates parent directory if missing.
#[test]
fn credentials_save_creates_parent_dir() {
    let dir = TempDir::new().unwrap();
    let nested = dir.path().join("deep").join("nested").join("creds.json");

    let creds = Credentials {
        private: "dGVzdA==".into(),
        certificate: "dGVzdA==".into(),
        id: "client-2".into(),
    };
    CredentialsStorage::save_to(&creds, &nested).unwrap();
    assert!(nested.exists());
}

/// Helper functions exposed on Credentials.
#[test]
fn credentials_instance_methods() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("creds.json");

    let creds = Credentials {
        private: "dGVzdA==".into(),
        certificate: "dGVzdA==".into(),
        id: "client-3".into(),
    };
    creds.save_to(&path).unwrap();
    assert!(path.exists());

    let loaded = Credentials::load_from(&path).unwrap().unwrap();
    assert_eq!(loaded.id, "client-3");

    assert!(Credentials::delete(&path).is_ok());
    assert!(!path.exists());
}

/// Credentials struct implements Debug and Clone.
#[test]
fn credentials_impl_debug_and_clone() {
    let creds = Credentials {
        private: "dGVzdA==".into(),
        certificate: "dGVzdA==".into(),
        id: "client-4".into(),
    };
    let _cloned = creds.clone();
    let _debug = format!("{creds:?}");
}
