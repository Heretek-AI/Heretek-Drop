// Copyright (C) 2025 Heretek-Drop contributors
// SPDX-License-Identifier: AGPL-3.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("src/ui/app-window.slint")?;
    Ok(())
}
