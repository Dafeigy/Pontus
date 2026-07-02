<h1 align="center">
  <img src="app-icon.png" width="64" alt="Pontus" />
  <br>Pontus
</h1>

<p align="center">
  <strong>Litellm Admin Desktop Client</strong><br>
  Built with Tauri v2 + Vue 3. User management, model connectivity testing, and LLM chat Playground.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vuedotjs" />
  <img src="https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript" />
  <img src="https://img.shields.io/badge/Rust-2021-000000?logo=rust" />
  <img src="https://img.shields.io/badge/Tailwind-4-06B6D4?logo=tailwindcss" />
</p>

---

## Features

- **First-run Setup** — Configure Litellm API endpoint & key, with optional SMTP mail service
- **User Dashboard** — View user list, spend, key count, with local caching
- **Invite Users** — Create Litellm user → generate invite link → send email automatically
- **Model List** — Browse models grouped by Access Group, test connectivity individually or in batch
- **Playground** — Select a model, stream chat with real-time Markdown rendering
- **Settings** — Update API config, SMTP parameters, light/dark theme toggle
- **Local Cache** — GET responses (users, models) are persisted to localStorage with daily auto-refresh

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop Framework | Tauri v2 |
| Frontend | Vue 3 + TypeScript + Vite |
| UI | shadcn-vue (reka-ui) + Tailwind CSS v4 |
| State Management | Pinia + tauri-plugin-store (persistence) |
| Charts | @unovis/vue |
| Rust HTTP | reqwest (blocking) |
| Rust Email | lettre (SMTP) |
| Streaming Chat | SSE → Tauri event emit → Vue listen |

## Project Structure

```
pontus/
├── src/                          # Vue frontend
│   ├── App.vue                   # Root component (loading / init wizard / layout)
│   ├── main.ts                   # Entry point, mounts Pinia + Router
│   ├── router/index.ts           # Hash route config
│   ├── stores/app.ts             # Global app state (theme, config)
│   ├── composables/              # Composable functions
│   │   └── useCachedFetch.ts     # Generic localStorage daily cache
│   ├── views/                    # Page views
│   │   ├── DashboardView.vue     # User dashboard
│   │   ├── InviteView.vue        # Invite users
│   │   ├── ModelsView.vue        # Model list & connectivity testing
│   │   ├── PlayGroundView.vue    # LLM chat Playground
│   │   ├── SettingsView.vue      # System settings
│   │   ├── TeamsView.vue         # Team management (placeholder)
│   │   └── HomeView.vue          # Home
│   ├── components/               # Components
│   │   ├── InitialSetup.vue      # First-run two-step wizard
│   │   ├── AppSidebar.vue        # Sidebar
│   │   ├── SiteHeader.vue        # Top header
│   │   ├── NavMain.vue           # Main navigation
│   │   ├── NavUser.vue           # User area
│   │   └── ui/                   # shadcn-vue UI primitives (50+)
│   └── assets/main.css           # Tailwind + CSS variable theme
│
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri config
│   └── src/
│       ├── main.rs               # Hide console on Windows
│       ├── lib.rs                # Plugin registration + command mounts
│       └── commands/
│           ├── config.rs         # App config read/write (tauri-plugin-store)
│           ├── email.rs          # SMTP email sending
│           ├── email_template.rs # Invite email HTML template
│           └── litellm/          # Litellm API client
│               ├── mod.rs        # Shared utilities + re-export
│               ├── types.rs      # Data type definitions
│               ├── users.rs      # User CRUD / invitations
│               └── models.rs     # Model list / testing / streaming chat
```

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.70
- Windows: [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) (C++ desktop development)

### Install Dependencies

```bash
npm install
```

### Development

```bash
# Frontend only (browser)
npm run dev

# Full Tauri desktop app
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

Output: `src-tauri/target/release/pontus.exe`.

## Config Storage

App configuration is persisted locally via tauri-plugin-store, including:
- Litellm server URL + API Key
- SMTP host / port / username / password / sender email
- Theme preference (light / dark)
