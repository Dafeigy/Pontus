# Implementation Plan: Litellm Manager Desktop App

## Architecture Overview

```
Frontend (Vue 3)                    Rust Backend (Tauri v2)
┌─────────────────────┐            ┌──────────────────────────┐
│ HomeView             │            │ Commands:                 │
│  ├─ InviteUserDialog │──invoke──▶│  ├─ invite_user           │
│  │   (email, alias,  │            │  │  1. POST /user/new     │
│  │    role selection) │            │  │  2. POST /invitation/..│
│                       │            │  │  3. Build email HTML  │
│ SettingsView         │            │  │  4. Send via SMTP      │
│  ├─ SMTP config form │──invoke──▶│  ├─ get_users             │
│  ├─ LiteLLM host     │            │  ├─ load_config           │
│  ├─ Theme toggle     │            │  ├─ save_config           │
│  ├─ Reset API Key    │            │  └─ reset_api_key         │
│                       │            │                          │
│ DashboardView        │            │ Config:                   │
│  ├─ Users table      │──invoke──▶│  JSON file in app data dir│
│  ├─ Pagination       │            │  (~/.pontus/config.json)  │
│                       │            │                          │
│ FirstRunSetup        │            │ Dependencies:             │
│  └─ API Key prompt   │            │  reqwest, lettre,         │
└─────────────────────┘            │  serde, serde_json        │
                                   └──────────────────────────┘
```

## Step-by-Step Plan

### Step 1: Rust Backend — Add Dependencies

**File:** `src-tauri/Cargo.toml`

Add these dependencies:
- `reqwest = { version = "0.12", features = ["json"] }` — HTTP client for LiteLLM API
- `lettre = { version = "0.11", features = ["builder"] }` — SMTP email sending
- `tokio = { version = "1", features = ["full"] }` — Already a transitive dep via Tauri; needed for async runtime
- `dirs = "5"` — Cross-platform app data directory

### Step 2: Rust Backend — Config Module

**New file:** `src-tauri/src/config.rs`

Store all settings in a JSON file at the OS-appropriate config directory:
- Windows: `%APPDATA%/com.cybersh1t.tauri-app/config.json`
- macOS: `~/Library/Application Support/com.cybersh1t.tauri-app/config.json`
- Linux: `~/.config/com.cybersh1t.tauri-app/config.json`

**Config struct:**
```rust
pub struct Config {
    pub api_key: Option<String>,          // sk-...
    pub smtp_host: String,                // smtp.example.com
    pub smtp_port: u16,                   // default 465
    pub smtp_sender_email: String,        // sender@example.com
    pub smtp_username: String,            // SMTP auth username
    pub smtp_password: String,            // SMTP auth password
    pub litellm_host: String,             // http://example.com
}
```

**Functions:**
- `load_config()` → `Config` (with defaults, `api_key` as `None`)
- `save_config(config: &Config)` → writes to JSON file, creating dirs as needed

### Step 3: Rust Backend — Tauri Commands

**File:** `src-tauri/src/lib.rs` (rewrite)

Commands to register:

| Command | Signature | Description |
|---------|-----------|-------------|
| `load_config` | `() -> Config` | Load config from file |
| `save_config` | `(Config) -> ()` | Save config to file |
| `reset_api_key` | `() -> ()` | Set api_key to None in config |
| `invite_user` | `(String, String, String) -> Result<String, String>` | Full invite flow: create user → invitation → send email. Args: email, alias, role. Returns success message. |
| `get_users` | `() -> Result<Vec<User>, String>` | Fetch user list from LiteLLM |

**`invite_user` flow:**
1. Load config
2. POST `{litellm_host}/user/new` → get `user_id` and `key` from response
3. POST `{litellm_host}/invitation/new` → get `id` from response
4. Build invitation link: `{litellm_host}/ui?invitation_id={id}`
5. Build email HTML using the template from requirements
6. Send email via SMTP (SSL, port 465) using `lettre`
7. Return success/error message

**Response structs (serde deserialization):**
- `CreateUserResponse` — from POST /user/new (need: `user_id`, `key`)
- `CreateInvitationResponse` — from POST /invitation/new (need: `id`)
- `UserListResponse` — from GET /user/list (need: `users[]`)

### Step 4: Frontend — Add Missing shadcn-vue Components

Need to add via MCP:
- **Dialog** — for the invite user form
- **Toast/Sonner** — for success/error feedback (optional but helpful — use existing components or simple alert)

### Step 5: Frontend — Settings Store (Composable)

**New file:** `src/composables/useConfig.ts`

A reactive composable that:
- On mount: calls `invoke('load_config')` to get current config
- Provides reactive refs for all settings fields
- Provides `save()` method that calls `invoke('save_config', config)`
- Provides `resetApiKey()` method
- Handles first-run detection (api_key is null → show setup)

### Step 6: Frontend — HomeView + InviteUserDialog

**Modify:** `src/views/HomeView.vue`

Features:
- Center-aligned, minimal layout
- "Invite User" button 
- Click opens Dialog with:
  - Email input (required, validated)
  - User alias input (required)
  - User role select dropdown (4 options, default: `internal_user_viewer`)
  - Submit button ("Invite")
- On submit: calls `invoke('invite_user', { email, alias, role })`
- Loading state during API call
- Success/error feedback

### Step 7: Frontend — SettingsView

**Modify:** `src/views/SettingsView.vue`

Sections:
1. **API Key** section:
   - Current status indicator (set/not set)
   - "Reset API Key" button (with confirmation dialog)

2. **LiteLLM Connection**:
   - `LITELLM_HOST` input

3. **SMTP Configuration**:
   - `SMTP_HOST` input
   - `SMTP_PORT` input (default 465)
   - `SMTP_SENDER_EMAIL` input
   - `SMTP_USERNAME` input
   - `SMTP_PASSWORD` input (password type)

4. **Appearance**:
   - Theme toggle button cycling: Auto → Dark → Light → Auto

5. **Save** button at bottom

### Step 8: Frontend — DashboardView

**Modify:** `src/views/DashboardView.vue`

Features:
- Table showing users from LiteLLM
- Columns: Email, Alias, Role, Created At, Key Count
- Pagination using the pagination info from API response
- Loading skeleton while fetching
- Refresh button

### Step 9: Frontend — First-Run API Key Setup

**Modify:** `src/App.vue` or create a new component

Logic:
- On app mount, load config
- If `api_key` is `None`, show a modal/dialog forcing user to enter API Key
- Once set, save config and proceed to normal app

### Step 10: Frontend — Theme Management

**New file or modify:** `src/composables/useTheme.ts`

- Theme modes: `'auto' | 'light' | 'dark'`
- Store preference in config (persist in config.json)
- On change: toggle `.dark` class on `<html>`, or remove it for auto to let OS preference decide
- Auto mode: listen to `prefers-color-scheme` media query

---

## Data Flow Summary

```
User clicks "Invite User"
  → Dialog opens (HomeView)
  → User fills email, alias, role
  → Frontend invokes Rust command `invite_user`
    → Rust loads config (SMTP + API Key + Litellm Host)
    → Rust POSTs to LiteLLM `/user/new`
    → Rust POSTs to LiteLLM `/invitation/new`
    → Rust builds invitation link
    → Rust builds HTML email
    → Rust sends email via SMTP
    → Returns success/failure to frontend
  → Frontend shows result to user

User opens Settings
  → Config loaded from file
  → User edits SMTP/Litellm settings, theme
  → User clicks Save
  → Config saved to file

User opens Dashboard
  → Frontend invokes `get_users`
  → Rust GETs LiteLLM `/user/list`
  → Returns user list to frontend
  → Table renders with pagination
```

## Files to Create/Modify

### New files:
- `src-tauri/src/config.rs` — Config persistence
- `src/composables/useConfig.ts` — Frontend config store
- `src/composables/useTheme.ts` — Theme management
- `src/components/InviteUserDialog.vue` — Invite dialog component
- `src/components/ui/dialog/*` — shadcn-vue dialog components (via MCP)

### Modified files:
- `src-tauri/Cargo.toml` — Add Rust deps
- `src-tauri/src/lib.rs` — Commands (full rewrite)
- `src/views/HomeView.vue` — Invite button + dialog
- `src/views/SettingsView.vue` — Full settings form
- `src/views/DashboardView.vue` — Users table
- `src/App.vue` — First-run check + theme initialization
