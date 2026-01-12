# Symlink Manager

A cross-platform desktop application for managing symbolic links across multiple directories. Built with Tauri, Vue 3, and Rust.

**[User Manual](docs/MANUAL.md)** · **[Download](https://github.com/flytoapp/symlink-manager/releases)**

## Features

- **Profile-based organization** - Group related symlink configurations into profiles
- **Multiple sources per profile** - Manage symlinks from different source directories to a common target
- **Visual status indicators** - See at a glance which items are active, inactive, broken, or have conflicts
- **Dark mode support** - Automatic theme detection with manual override
- **Cross-platform** - Works on macOS, Windows, and Linux

## Use Cases

- Selectively enabling game mods or plugins
- Symlinking dotfiles from a repository to your home directory
- Organizing shared configuration files
- Any workflow requiring toggling symlinks on/off

## Installation

Download the latest release for your platform from the [Releases](https://github.com/flytoapp/symlink-manager/releases) page:

| Platform | File | Notes |
|----------|------|-------|
| **Windows** | `.msi` | Standard installer |
| **macOS (Apple Silicon)** | `aarch64.dmg` | M1/M2/M3/M4 Macs |
| **macOS (Intel)** | `x64.dmg` | Older Intel Macs |
| **Linux (Debian/Ubuntu)** | `.deb` | `sudo dpkg -i <file>` |
| **Linux (Other)** | `.AppImage` | Universal, run directly |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- Platform-specific dependencies: [Tauri prerequisites](https://tauri.app/start/prerequisites/)

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

Built applications are output to `src-tauri/target/release/bundle/`.

## Windows Notes

Creating symbolic links on Windows requires either:
- **Developer Mode enabled** (Settings → System → For developers) - recommended
- Running the application as Administrator

## Concepts

- **Profile**: A named configuration with a default target path (e.g., `~/.config`) and one or more sources
- **Source**: A directory containing items you want to symlink (e.g., your dotfiles repo's `config/` folder)
- **Item**: An individual file or folder within a source that can be enabled (symlinked) or disabled

## Tech Stack

- **Frontend**: Vue 3, TypeScript, Pinia, Tailwind CSS v4
- **Backend**: Rust, Tauri v2
- **Icons**: Lucide Vue

## License

MIT
