# Migration Plan: litellm-manager → Pontus

## Summary
Migrate all litellm-manager functionality into the Pontus shadcn-vue template while preserving Pontus's superior UI shell (sidebar, header, theming).

## Key Architectural Decisions

1. **Keep Pontus UI framework** — shadcn-vue sidebar system, Reka UI primitives, Tailwind v4, @tabler/icons-vue
2. **Port litellm-manager's Pinia store** — `src/stores/app.ts` for reactive state
3. **Port Rust backend entirely** — all 4 command modules + dependencies
4. **Adapt litellm-manager views** to use Pontus's shadcn-vue components instead of radix-vue
5. **Add InitialSetup flow** integrated into App.vue's existing loading pattern
6. **Add vue-sonner** for toast notifications (not present in Pontus yet)

## Step-by-Step Plan

### Phase 1: Dependencies & Configuration

**A. Update `package.json`** — add missing npm deps:
- `pinia` (^2.3.0) — state management
- `vue-sonner` (^1.3.0) — toast notifications
- `@tauri-apps/plugin-store` (^2.2.0) — persistent config storage
- Add `Sonner` component to `src/components/ui/`

**B. Update `src-tauri/Cargo.toml`** — add Rust deps:
- `tauri-plugin-store` = "2"
- `reqwest` = { version = "0.12", features = ["json", "blocking"] }
- `lettre` = { version = "0.11", features = ["builder", "smtp-transport", "tokio1-native-tls"] }
- `tokio` = { version = "1", features = ["full"] }

**C. Update `src-tauri/tauri.conf.json`**:
- Change productName to "Pontus", identifier to "com.NUL4i.pontus"
- Set bundle.active to true
- Increase default window size to 1100x750

### Phase 2: Rust Backend (Complete Rewrite of lib.rs + New Command Modules)

**D. Create `src-tauri/src/commands/` directory** with files:
- `mod.rs` — module declarations
- `config.rs` — AppConfig struct + CRUD commands using tauri-plugin-store
- `litellm.rs` — Litellm API client (create_user, generate_invitation, list_users)
- `email.rs` — SMTP email sending via lettre
- `email_template.rs` — HTML email template builder

**E. Rewrite `src-tauri/src/lib.rs`**:
- Register all 9 Tauri commands
- Register tauri-plugin-store plugin
- Remove old `greet` command

### Phase 3: Frontend State & Core Setup

**F. Create `src/stores/app.ts`** — Pinia store:
- AppConfig interface (apiKey, litellmHost, smtpHost, smtpPort, smtpSenderEmail, smtpUsername, smtpPassword, theme)
- isInitialized ref
- applyTheme() function using classList
- Theme watcher

**G. Update `src/main.ts`**:
- Add `createPinia()` import
- Add `app.use(pinia)` before router

**H. Create `src/components/ui/sonner/`** — vue-sonner wrapper:
- `Sonner.vue` and `index.ts`

### Phase 4: Initial Setup Wizard

**I. Create `src/components/InitialSetup.vue`**:
- Two-step wizard (Litellm config → SMTP config)
- Step indicators with numbered circles
- Uses Pontus shadcn-vue Input, Button, Label, Card
- Uses lucide-vue-next icons (ArrowRight, Check)

### Phase 5: App Shell Updates

**J. Rewrite `src/App.vue`**:
- Add loading state + initialization check on mount
- Show InitialSetup when not initialized
- Show existing AppLayout (sidebar + header) when initialized
- Add `<Sonner />` for toast notifications
- Update nav items: Invite, Dashboard, Settings

**K. Update `src/router/index.ts`**:
- Change "/" redirect from "/home" to "/invite"
- Change "/home" route to "/invite" (InviteView)
- Keep /dashboard and /settings routes

### Phase 6: Views Implementation

**L. Rewrite `src/views/HomeView.vue` → content becomes InviteView**:
- User invitation form with name, email, role select
- Two-step process indicator (create user → send email)
- Uses shadcn-vue Input, Button, Label, Card, Select
- Form validation

**M. Rewrite `src/views/DashboardView.vue`**:
- User table with TanStack Vue Table (already a dependency)
- Columns: name, email, role, key count, spend, created date
- Refresh button, pagination
- Skeleton loading states
- Empty state handling

**N. Rewrite `src/views/SettingsView.vue`**:
- Litellm API config section (host URL, API key, reset button)
- SMTP config section (host, port, sender, username, password)
- Theme toggle (dark mode switch)
- Save button
- Load config from store on mount

### Phase 7: UI Components Needed

**O. Verify existing shadcn-vue components** — Pontus already has:
- Button, Input, Label, Select, Card, Avatar, Badge, Checkbox, Separator, Sheet, Skeleton — ✅
- Table components — NOT present, need to add from shadcn-vue
- Switch component — NOT present, need to add from shadcn-vue
- Dialog component — NOT present (may need for confirmations)

**P. Create missing UI components** — add what's needed:
- `src/components/ui/table/` — Table, TableHeader, TableBody, TableRow, TableHead, TableCell
- `src/components/ui/switch/` — Switch component
- `src/components/ui/sonner/` — Sonner wrapper

### Phase 8: Icons Strategy

**Q. Icon library**: Use `@tabler/icons-vue` (primary) since Pontus already uses it, with `lucide-vue-next` available as secondary. The sidebar already uses Tabler icons. For view-specific icons, use Tabler equivalents of the lucide icons from litellm-manager.

### Phase 9: Cleanup & Polish

**R. Remove unused code**:
- Remove old HomeView (replaced by InviteView)
- Remove old stub DashboardView and SettingsView (rewritten)
- Remove placeholder nav data from App.vue

**S. Final verification**:
- `npm run build` type-check
- Cargo check for Rust backend
- Verify all Tauri commands are registered correctly

## File Change Summary

| File | Action |
|------|--------|
| `package.json` | Edit — add deps |
| `src-tauri/Cargo.toml` | Edit — add Rust deps |
| `src-tauri/tauri.conf.json` | Edit — branding |
| `src/main.ts` | Edit — add pinia |
| `src/App.vue` | Rewrite — init flow + nav |
| `src/router/index.ts` | Edit — invite route |
| `src/stores/app.ts` | Create — Pinia store |
| `src/components/InitialSetup.vue` | Create — wizard |
| `src/components/ui/sonner/*` | Create |
| `src/components/ui/table/*` | Create |
| `src/components/ui/switch/*` | Create |
| `src/views/HomeView.vue` | Rewrite → InviteView |
| `src/views/DashboardView.vue` | Rewrite |
| `src/views/SettingsView.vue` | Rewrite |
| `src-tauri/src/lib.rs` | Rewrite |
| `src-tauri/src/main.rs` | Edit — crate name |
| `src-tauri/src/commands/*` | Create (5 files) |
