// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Heretek-Drop binary entry point.

#![cfg_attr(all(not(debug_assertions), target_os = "linux"), deny(missing_docs))]

use anyhow::Context;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize structured logging from `RUST_LOG` env var, defaulting to `info`.
fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_level(true))
        .init();
}

fn main() -> anyhow::Result<()> {
    init_logging();

    let app = heretek_drop::App::new().context("failed to initialize app")?;
    app.run().context("app crashed")?;
    Ok(())
}
