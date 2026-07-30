# Heretek-Drop Desktop

The native Slint + Rust client for Drop.

## Layout

```
apps/desktop/
├── Cargo.toml             # binary crate (heretek-drop)
├── build.rs               # slint_build
├── src/
│   ├── main.rs            # entry point
│   ├── lib.rs             # library root
│   ├── app.rs             # App lifecycle
│   ├── config.rs          # Config loading
│   ├── state.rs           # App state
│   └── ui/
│       ├── app-window.slint       # Slint root
│       ├── pages/login.slint
│       ├── pages/library.slint
│       ├── pages/game.slint
│       ├── pages/downloads.slint
│       └── pages/settings.slint
├── crates/
│   ├── shared/            # cross-crate errors
│   ├── protocol/          # HTTP + JWT
│   ├── auth/              # auth flow + credentials
│   ├── database/          # SQLite (rustbreak)
│   ├── download_manager/  # chunked downloads
│   └── process_manager/   # game process spawn
├── flatpak/
│   ├── app.heretek.drop.yml
│   ├── app.heretek.drop.desktop
│   └── app.heretek.drop.mimeinfo.xml
└── icons/                 # PNG icons (TBD)
```

## Build

```bash
# Build workspace
cargo build --workspace

# Run
cargo run --bin heretek-drop

# Tests
cargo test --workspace

# Live UI iteration (no recompile)
cargo install slint-viewer
slint-viewer apps/desktop/src/ui/app-window.slint

# Flatpak
cd apps/desktop
flatpak-builder build flatpak/app.heretek.drop.yml --user --install
```

## Next steps (Wave 2 → Wave 3)

- [ ] Wire Slint callbacks to Rust handlers in `app.rs`
- [ ] Replace stub auth flow with real Drop API calls
- [ ] Implement library fetch + cache invalidation
- [ ] Implement download queue UI
- [ ] Implement cover art loading + image caching
- [ ] Add app icon (PNG, 256x256 minimum)
- [ ] Add desktop file completion tests
