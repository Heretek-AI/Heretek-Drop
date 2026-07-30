---
name: rust-gui-patterns
description: Rust + Slint + Tokio patterns. Event-loop bridging, image loading, async UI. Use when integrating Rust with Slint UI.
---
> **Currency**: This skill was last verified against the workspace dependencies in `Cargo.toml` (slint 1.13, tokio 1.42, reqwest 0.12, jsonwebtoken 9.3, thiserror 2.0, anyhow 1.0). Before relying on API shapes from this file, check current docs.rs for the relevant crate. If versions have drifted, update this note or refresh the skill content.


# Rust + Slint + Tokio Patterns

Heretek-Drop uses Slint for UI, Tokio for async, and Rust for everything else. These patterns handle the interop.

## Pattern: Tokio task → Slint UI update

Use `slint::spawn_local` for the UI thread, `tokio::spawn` for background work.

```rust
use slint::spawn_local;

// Inside main.rs (UI thread)
let ui_weak = ui.as_weak();
let handle = tokio::spawn(async move {
    let result = fetch_library().await;
    let ui_weak = ui_weak.clone();
    spawn_local(async move {
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_library(result);
        }
    }).unwrap();
});
```

## Pattern: Bounded channels for high-frequency events

Download progress fires at 60Hz. Use `flume::bounded` (or `tokio::sync::mpsc::channel`).

```rust
let (tx, mut rx) = tokio::sync::mpsc::channel::<DownloadProgress>(64);

// Background producer
tokio::spawn(async move {
    while let Some(p) = progress_stream().await {
        if tx.send(p).await.is_err() { break; }
    }
});

// UI consumer (sync context)
slint::spawn_local(async move {
    while let Some(p) = rx.recv().await {
        ui.set_progress(p.into());
    }
}).unwrap();
```

Bounded channels prevent memory bloat if the UI thread hangs.

## Pattern: Image loading from network

```rust
pub async fn load_cover_async(url: String) -> Result<slint::Image> {
    let bytes = reqwest::get(&url).await?.bytes().await?;
    // slint::Image::load_from_data is sync and fast (no I/O)
    let img = slint::Image::load_from_data(bytes.as_ref())
        .ok_or_else(|| anyhow!("Failed to decode image"))?;
    Ok(img)
}
```

## Pattern: Slint component stored in struct

```rust
pub struct App {
    ui: AppWindow,
    state: Arc<Mutex<AppState>>,
    runtime: tokio::runtime::Runtime,
}

impl App {
    pub fn new() -> Result<Self> {
        let ui = AppWindow::new()?;
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let state = Arc::new(Mutex::new(AppState::default()));
        Ok(Self { ui, state, runtime })
    }
    
    pub fn run(self) -> Result<()> {
        let ui_weak = self.ui.as_weak();
        self.ui.on_login(move |username, password| {
            let ui_weak = ui_weak.clone();
            spawn_local(async move {
                match do_login(username, password).await {
                    Ok(_) => { /* nav to library */ }
                    Err(e) => { /* show error */ }
                }
            }).unwrap();
        });
        self.ui.run()?;
        Ok(())
    }
}
```

## Pattern: Slint data from Rust struct

```rust
#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub title: String,
    pub cover_url: String,
}

impl From<Game> for slint_generated::Game {
    fn from(g: Game) -> Self {
        Self {
            id: g.id as i32,
            title: g.title.into(),
            cover_url: g.cover_url.into(),
        }
    }
}
```

## Pattern: Long-running operations

```rust
pub async fn download_game(
    url: String,
    dest: PathBuf,
    progress_tx: mpsc::Sender<DownloadProgress>,
) -> Result<()> {
    let mut stream = reqwest::get(&url).await?.bytes_stream();
    let mut file = tokio::fs::File::create(&dest).await?;
    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        progress_tx.send(DownloadProgress {
            downloaded,
            ..Default::default()
        }).await?;
    }
    Ok(())
}
```

## Pattern: Configuration

Use `directories` crate to find user config dir:

```rust
use directories::ProjectDirs;

let dirs = ProjectDirs::from("dev", "heretek", "drop")
    .ok_or_else(|| anyhow!("No config dir"))?;
let config_path = dirs.config_dir().join("config.toml");
let creds_path = dirs.config_dir().join("credentials.json");
```

## Pattern: Structured logging

```rust
use tracing::{info, error, instrument};

#[instrument(skip(password))]
pub async fn login(username: String, password: String) -> Result<Credentials> {
    info!("login attempt");
    let res = client.post(format!("{base}/auth/initiate"))
        .json(&json!({ "username": username }))
        .send().await?;
    if !res.status().is_success() {
        error!(status = %res.status(), "login failed");
        return Err(anyhow!("login failed"));
    }
    // ...
}
```

## Don't

- Don't call `unwrap()` in async fn bodies — propagate errors.
- Don't `block_in_place` from the UI thread — use `tokio::task::spawn_blocking`.
- Don't store `slint::Image` in `Mutex` — wrap in `Arc` instead.
- Don't use `std::sync::Mutex` from async code — use `tokio::sync::Mutex`.
- Don't allocate strings in tight loops — use `Cow<str>` or reuse `String`.
- Don't mix `println!` with `tracing` — pick one per binary.
