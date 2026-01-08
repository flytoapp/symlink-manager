# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development (starts both Vite dev server and Tauri)
npm run tauri dev

# Production build
npm run build              # TypeScript check + Vite build
npm run tauri build        # Full Tauri app bundle

# Type checking only
npx vue-tsc --noEmit

# Regenerate app icons from source
npm run generate:icons
```

## Architecture

This is a **Tauri v2 + Vue 3** desktop application for managing symbolic links across multiple directories.

### Core Concepts

- **Profile**: A configuration grouping with a `basePath` (default target) and multiple sources
- **Source**: A directory containing items to be symlinked, with optional custom `targetPath`
- **Item**: An individual file/folder within a source that can be enabled (symlinked) or disabled

### Frontend (Vue 3 + TypeScript)

- **State**: Single Pinia store at `src/stores/appStore.ts` manages all app state
- **Composables**: `src/composables/` - `useItems.ts` handles symlink toggling, `useProfiles.ts`/`useSources.ts` for CRUD operations
- **Path alias**: `@/` maps to `src/` directory
- **Styling**: Tailwind CSS v4 with dark mode support

### Backend (Rust/Tauri)

- **Entry**: `src-tauri/src/lib.rs` - Tauri builder setup and command registration
- **Commands**: `src-tauri/src/commands/` - Tauri IPC handlers
  - `config.rs` - Profile/source CRUD, persists to `config.json` in app data directory
  - `symlinks.rs` - Creates/removes symlinks, includes Windows privilege handling
  - `filesystem.rs` - Directory listing and path validation
- **Models**: `src-tauri/src/models/` - `AppConfig`, `Profile`, `Source` with serde serialization
- **State**: `AppState` struct holds config in `Mutex<AppConfig>` and config file path

### Frontend-Backend Communication

All IPC uses `@tauri-apps/api/core` `invoke()`. Commands are defined with `#[tauri::command]` and registered in `lib.rs`. Types use `camelCase` in JSON (via `#[serde(rename_all = "camelCase")]`).

### Platform-Specific Code

Windows symlink creation requires either Developer Mode or admin privileges. The `check_symlink_permissions` command tests this at startup. Windows-specific code uses `#[cfg(windows)]` conditionals and the `winreg` crate for registry access.
