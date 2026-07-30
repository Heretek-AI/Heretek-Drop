---
name: drop-protocol
description: Drop REST API at /api/v1/client/* with ES384 JWT auth. Use when implementing any client API call, auth flow, or schema.
---

# Drop Protocol — REST API shapes

Drop is an open-source game distribution platform. The client API lives at `/api/v1/client/*` and uses ES384 JWT auth.

## Base URL

```
{base_url}/api/v1/client/*
```

Default port: 3433. Configurable via `~/.config/heretek-drop/config.toml`:
```toml
[server]
base_url = "https://drop.example.com"
```

## Auth flow

### 1. Initiate
```
POST /api/v1/client/auth/initiate
→ {
    "id": "client-uuid",
    "redirect_url": "https://drop.example.com/auth/callback?code=...",
    "expires_at": "2025-01-01T00:00:00Z"
  }
```

Client opens the `redirect_url` in the system browser. User authenticates (password / passkey / OIDC).

### 2a. Browser deep-link flow
After auth, Drop server redirects to `drop://auth/callback?code=...`. Client handles via platform deep-link.

### 2b. Code flow (alternative)
```
POST /api/v1/client/auth/code
→ { "code": "abc123", "expires_at": "..." }
```
Then `GET /api/v1/client/auth/code/ws` (WebSocket) waits for the auth code.

### 3. Handshake
```
POST /api/v1/client/auth/handshake
Body: { "code": "abc123" }
→ {
    "private": "base64-es384-private-key",
    "certificate": "base64-es384-cert",
    "id": "client-uuid"
  }
```

Client stores `{private, certificate, id}` in `~/.config/heretek-drop/credentials.json`.

### 4. Authenticated requests
```
GET /api/v1/client/user
Authorization: JWT <client_id> <ES384_signed_JWT>
```

JWT claims: `{ nbf: <now>, exp: <now+10s> }`. Sign with the user's private key.

## Endpoints

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/api/v1/` | Healthcheck: `{ "appName": "Drop", ... }` |
| `POST` | `/api/v1/client/auth/initiate` | Begin auth flow |
| `POST` | `/api/v1/client/auth/code` | Request auth code |
| `GET` | `/api/v1/client/auth/code` | Poll auth code |
| `GET` | `/api/v1/client/auth/code/ws` | WebSocket auth code delivery |
| `POST` | `/api/v1/client/auth/handshake` | Exchange code for credentials |
| `GET` | `/api/v1/client/user` | `User` profile |
| `GET` | `/api/v1/client/user/library` | `Vec<Game>` (owned games) |
| `GET` | `/api/v1/client/game/{id}` | `Game` detail |
| `GET` | `/api/v1/client/game/{id}/versions` | `Vec<VersionDownloadOption>` |
| `GET` | `/api/v1/client/game/{id}/version/{versionId}` | Single version detail |
| `GET` | `/api/v1/client/saves` | All save settings |
| `GET` | `/api/v1/client/saves/{gameId}` | Saves for a game |
| `POST` | `/api/v1/client/saves/{gameId}` | Upload save |
| `DELETE` | `/api/v1/client/saves/{gameId}/{slotIndex}` | Delete save slot |

## Types

```rust
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub client_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub cover_url: String,
    pub banner_url: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub genres: Vec<String>,
    pub platforms: Vec<String>,
    pub age_rating: Option<String>,
    pub release_date: Option<String>,
    pub average_playtime: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct VersionDownloadOption {
    pub id: u32,
    pub version: String,
    pub size_bytes: u64,
    pub download_url: String,
    pub checksum: String,
    pub platform: String,
}
```

## Error responses

```json
{
  "statusCode": 401,
  "statusMessage": "Unauthorized",
  "message": "Invalid JWT signature"
}
```

| Code | Meaning |
|---|---|
| 401 | JWT invalid/expired/missing |
| 403 | User lacks permission |
| 404 | Resource not found |
| 429 | Rate-limited (retry with backoff) |
| 500 | Server error (retry with backoff) |
| 503 | Server unavailable (retry with longer backoff) |

## Don't

- Don't cache JWT for longer than 10 seconds — sign per-request, not per-session.
- Don't trust `role` field from API — it's a UI hint, not an access control.
- Don't store private key in plaintext on disk — use OS keyring via `keyring` crate.
