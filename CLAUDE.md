# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
npm run dev          # Start Vite dev server (port 1420)
npm run build        # Type-check (vue-tsc --noEmit) then build for production
npm run preview      # Preview production build locally
npm run tauri        # Tauri CLI — use `npm run tauri dev` to run the full Tauri desktop app
```

TypeScript is configured with `strict: true`, `noUnusedLocals: true`, and `noUnusedParameters: true` — unused imports/variables will cause build failures.

## Architecture Overview

**Tauri v2 desktop app** with a **Vue 3 + TypeScript** frontend. The Rust backend (`src-tauri/`) serves the Vite-built frontend as a webview.

### Frontend (Vue 3 + Vite)

- **Entry point**: `src/main.ts` → mounts `src/App.vue` with Vue Router
- **Routing**: Hash-based history (`createWebHashHistory`) — required for Tauri's file:// protocol. Routes defined in `src/router/index.ts` with lazy-loaded views.
- **Views**: `src/views/` — page-level components (HomeView, SettingsView, DashboardView)
- **Components**: `src/components/` — app shell components (AppSidebar, SiteHeader, NavMain, NavSecondary, NavUser) plus a `ui/` subdirectory with shadcn-vue primitives
- **Path alias**: `@/` resolves to `src/`
- **No composables directory yet** — composables would go in `src/composables/` (per `components.json` aliases)

### UI System (shadcn-vue)

- shadcn-vue components live in `src/components/ui/` and are built on **Reka UI** (headless component primitives)
- The `cn()` utility in `src/lib/utils.ts` merges Tailwind classes via `clsx` + `tailwind-merge`
- Theming uses CSS variables (light/dark via `.dark` class) defined in `src/assets/main.css`
- **Tailwind CSS v4** with `tw-animate-css` plugin for animations
- **Icons**: `@tabler/icons-vue` for app icons, `@lucide/vue` available as shadcn's default icon library
- **Charts**: `@unovis/vue` charting library is available

### Rust Backend (Tauri v2)

- `src-tauri/src/main.rs` — binary entry point, hides console window on Windows release builds
- `src-tauri/src/lib.rs` — app setup with `tauri-plugin-opener` and a single `greet` command
- `src-tauri/tauri.conf.json` — Tauri configuration: dev server on port 1420, frontend dist in `../dist`, CSP disabled

### Data Flow

Navigation data (sidebar items, user info) is defined as a reactive object in `App.vue` and passed as props down to sidebar components. Route changes are handled client-side via Vue Router with hash history. Tauri IPC commands are defined in `lib.rs` and invoked from the frontend via `@tauri-apps/api`.
