# Testing Heretek-Drop

## Prerequisites

A running Drop server instance. You can use:

```bash
# Local development server (Docker)
docker run -p 3433:3433 -e DROP_ADMIN_TOKEN=dev-token ghcr.io/drop-oss/drop:nightly
```

Or connect to an existing instance by editing `~/.config/heretek-drop/config.toml`:

```toml
[server]
base_url = "https://your-drop-server.com"
```

## Unit Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p heretek-drop-protocol
cargo test -p heretek-drop-database
cargo test -p heretek-drop-auth
cargo test -p heretek-drop-download-manager
cargo test -p heretek-drop-process-manager

# Test with output
cargo test -- --nocapture
```

## Smoke Test (Manual)

Run against a real Drop server after building.

### 1. Build

```bash
cargo build --release
```

### 2. Run

```bash
cargo run --bin heretek-drop
```

Expected: Window opens with "Sign in to Drop" page.

### 3. Auth Flow

1. Click "Sign in via browser"
2. Your default browser opens a Drop auth page
3. Log in (create account or use admin token)
4. Browser redirects back to `drop://auth/callback?code=...` callback
5. App window navigates to "My Library" page

If the deep-link handler isn't wired yet (v0.1 limitation):

- Copy the auth code from the browser URL
- Use the code-based flow (not yet implemented in v0.1 — see v0.2)

### 4. Library Browse

After successful login:

- "My Library" page shows your owned games
- Click "Refresh" to reload from server
- A game card shows title, developer, and a Download button

### 5. Download

1. Click "Download" on a game
2. Progress bar appears in the Download queue
3. Download completes to `~/Games/Heretek-Drop/{game_id}.bin`
4. SHA-256 checksum verified against server

### 6. Settings

1. Navigate to Settings via top nav
2. View server URL (read-only in v0.1)
3. Browse download directory
4. Toggle theme (auto / light / dark)
5. Sign out returns to login page

### 7. Flatpak Install (v0.1 release target)

```bash
cd apps/desktop
flatpak-builder build flatpak/app.heretek.drop.yml --user --install
flatpak run app.heretek.drop
```

## CI Pipeline

See `.github/workflows/`:

- `ci.yml`: validation jobs run on every PR (+ scheduled)
- `release.yml`: tag-triggered flatpak build
- `codeql.yml`: Rust + Actions security scanning
- `osv-scanner.yml`: vulnerability scanning
- `editorconfig-ci.yml`: format compliance

## Known Test Gaps (v0.1)

- **JWT signing**: No automated test for ES384 signing (requires valid key pair)
- **Download manager**: Full end-to-end download not tested in CI (needs HTTP server)
- **Process manager**: Game launch not tested (needs real executable)
- **Slint UI**: No UI automation tests (Playwright / headless Slint not available)
- **Flatpak build**: Not tested in CI (needs flatpak-builder runtime)
- **Auth flow end-to-end**: Cannot test full browser flow in CI (needs browser automation)

## Coverage

Coverage measurement deferred to v0.2. See `.codecov.yml` and `.codecov.yml`
for coverage thresholds. Current status: unit tests only, run `cargo test`.
