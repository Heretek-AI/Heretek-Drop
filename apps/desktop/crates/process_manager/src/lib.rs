// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

//! Game process tracker.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod manager;
mod process;

pub use manager::ProcessManager;
pub use process::{GameProcess, LaunchSpec};
