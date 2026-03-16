# Godot Hub

A cross-platform Godot Engine version manager built with Rust and egui.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)

## Features

- **Version Management** — Browse, download, and delete Godot engine versions (Standard & Mono)
- **Real-time Download** — Streaming download with progress, cancellation, and retry support
- **Project Browser** — Scan your projects directory and list Godot projects
- **Custom Mirror** — Configure a custom mirror URL to accelerate downloads
- **Theme Support** — Dark and light themes

## Screenshots

> Coming soon.

## Installation

### Build from Source

**Prerequisites**: Rust 1.75+ and Cargo

```bash
git clone https://github.com/nedrw/godothub.git
cd godothub
cargo build --release
./target/release/gdhub
```

## Usage

1. **Download a version** — Click "⬇️ Download New Version" in the sidebar, select a Godot version from the list
2. **Launch Godot** — Click "▶ Run" on any installed version card
3. **Manage versions** — Use the "⋮" menu on each card to open the folder, toggle favorites, or remove a version
4. **Configure mirrors** — Go to Settings → Download Source → Custom, enter your mirror URL

### Configuration File

The app stores settings at:

| Platform | Path |
|----------|------|
| macOS    | `~/Library/Application Support/gdhub/config.json` |
| Linux    | `~/.config/gdhub/config.json` |
| Windows  | `%APPDATA%\gdhub\config.json` |

Godot versions are installed to `~/.gdhub/versions/` by default.

## Tech Stack

| Crate | Purpose |
|-------|---------|
| `eframe` / `egui` | Cross-platform GUI |
| `tokio` | Async runtime |
| `reqwest` | HTTP client (streaming download) |
| `serde` / `serde_json` | Config serialization |
| `zip` | ZIP extraction |
| `rfd` | Native file picker dialog |

## Project Structure

```
src/
├── main.rs           # Entry point
├── models/           # Data structures (GodotVersion, GodotInstall, GodotVariant)
├── state/            # Application state & configuration
├── services/         # GitHub API, downloader, launcher
├── ui/               # egui panels and components
└── utils/            # File utilities, region detection
```

## Documentation

- [Architecture](doc/ARCHITECTURE.md) — Module design, data models, async architecture, and known issues
- [UI Design](doc/UI_DESIGN.md) — Color system, component library, panel layouts
- [TODO](doc/TODO.md) — Prioritized backlog and known bugs

## Known Limitations

- macOS: launching installed Godot versions may fail (`.app` bundle path detection issue)
- Project management (New/Import/Open/Favorite) is not yet implemented
- Version search/filter in the download dialog is not functional
- Favorites and last-used timestamps are not persisted across restarts

## License

MIT © 2025 Godot Hub