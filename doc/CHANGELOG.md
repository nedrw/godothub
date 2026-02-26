# Changelog

All notable changes to Godot Hub will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub API integration for fetching real Godot version list
- Real download and extraction functionality
- File picker dialog for directory selection
- Version deletion with confirmation dialog
- State persistence on application close
- Theme switching support (Dark/Light/System)
- Keyboard shortcuts for common actions
- Project template selection on creation
- Auto-update checker for the application itself

### Changed
- Improve download manager with progress persistence
- Enhance error handling and user feedback
- Optimize performance for large version lists

### Fixed
- Download resume after application restart
- Platform-specific executable permissions

## [0.1.1] - 2025-01-16

### Added
- Sidebar optimization with application header and version display
- Navigation buttons with emoji icons for better recognition
- Statistics cards showing installed, available, and downloading counts
- Download button fixed at bottom of sidebar with hover tooltip
- Version management panel with card-based layout
- Version tags (Standard/Mono/Export Templates) with color coding
- Favorite marking for installed versions
- Operation menu for each installed version (Open Folder, Toggle Favorite, Remove)
- Version grouping by major version (Godot 4.x / 3.x)
- Download queue status indicator in download dialog
- Cancel all downloads button
- Search bar placeholder in download dialog
- Project scanning functionality to discover Godot projects
- Project validity detection and status tags
- Quick actions in projects panel (New, Import, Open Folder)
- Empty state guidance for projects panel
- Card-based settings layout with sections
- Directory quick-open buttons in settings
- Theme selection placeholder (Dark/Light/System)
- Technology stack display in about section
- Tooltips for all major UI elements

### Changed
- Redesigned sidebar with better visual hierarchy
- Improved version list display with card layout
- Enhanced project list with card layout and better information display
- Redesigned settings panel with grouped cards
- Optimized download dialog layout and grouping
- Improved button styles with consistent sizing and colors
- Better error messages and user feedback
- Enhanced accessibility with keyboard navigation support
- Updated ARCHITECTURE.md with comprehensive documentation
- Created TODO.md for development tracking
- Created UI_DESIGN.md for design guidelines

### Fixed
- Download progress bar now displays correctly
- Progress bar animation now works properly
- Version list refresh issue resolved
- UI component responsive layout improvements
- State borrowing issues in download callbacks
- Deprecated API usage (Frame::none, Frame::rounding, copied_text)
- Response ownership issues with hover tooltips
- Margin type mismatch (f32 to i8)

### Technical
- Updated egui API usage to 0.31
- Fixed all compilation errors and warnings
- Improved code organization and structure
- Added comprehensive code documentation
- Better error handling patterns

## [0.1.0] - 2025-01-15

### Added
- Initial project structure with Rust + eframe/egui
- Basic UI framework with sidebar and panels
- Version management panel with installed and available versions
- Project management panel (placeholder)
- Settings panel with directory configuration
- Download dialog for version selection
- Configuration persistence with JSON
- Godot version data models (GodotVersion, GodotInstall, GodotVariant)
- Application state management (AppState, AppConfig)
- Download service framework
- Godot launcher service with cross-platform support
- File utilities for directory management
- Basic error handling with thiserror and anyhow
- Logging system with env_logger
- Mock data for Godot versions
- Documentation (ARCHITECTURE.md)

### Features
- View installed Godot versions
- View available Godot versions from mock data
- Launch installed Godot versions
- Directory configuration for installations and projects
- Check for updates on startup option
- Basic project directory scanning
- Cross-platform support (Windows, macOS, Linux)

### Technical Stack
- Rust 2021 Edition
- eframe 0.31 for UI
- egui 0.31 for immediate mode GUI
- tokio 1.x for async runtime
- reqwest 0.12 for HTTP client
- serde 1.0 for serialization
- zip 2.0 for archive extraction
- chrono 0.4 for date/time
- thiserror 2.0 for custom errors
- anyhow 1.0 for error handling
- log 0.4 and env_logger 0.11 for logging
- dirs 6.0 for system directories

## [0.0.1] - 2025-01-10

### Added
- Project initialization with Cargo
- Basic project structure
- Initial commit

---

## Version History

| Version | Release Date | Description |
|---------|--------------|-------------|
| 0.1.1 | 2025-01-16 | UI optimization and bug fixes |
| 0.1.0 | 2025-01-15 | Initial release with basic features |
| 0.0.1 | 2025-01-10 | Project initialization |

## Release Notes

### Version 0.1.1 - UI Optimization Release

This release focuses on improving the user interface and user experience. Key improvements include:

1. **Sidebar Redesign**
   - Added application header with logo and version
   - Implemented navigation buttons with icons
   - Added statistics cards
   - Fixed download button position

2. **Version Management Improvements**
   - Card-based layout for better readability
   - Version tags with color coding
   - Favorite marking support
   - Operation menu for quick actions

3. **Project Management Enhancements**
   - Automatic project scanning
   - Project validity detection
   - Better empty state guidance

4. **Settings Panel Refinement**
   - Grouped card layout
   - Quick directory access
   - Theme selection placeholder

5. **Bug Fixes**
   - Fixed download progress bar display
   - Resolved various UI rendering issues
   - Fixed compilation warnings

### Version 0.1.0 - Initial Release

First functional release of Godot Hub with basic features:

- Version viewing and launching
- Directory configuration
- Mock version data
- Basic project scanning
- Cross-platform support

---

## Roadmap

### v0.2.0 (Planned: 2025-02-01)
- Real GitHub API integration
- Working download functionality
- Project creation wizard
- Version deletion feature

### v0.3.0 (Planned: 2025-02-15)
- Theme customization
- Keyboard shortcuts
- Enhanced error handling
- Project templates

### v0.4.0 (Planned: 2025-03-01)
- Auto-update checker
- Version comparison
- Multi-language support
- Plugin system foundation

### v1.0.0 (Planned: 2025-04-01)
- Complete feature set
- Full test coverage
- Performance optimization
- Cross-platform testing
- Stable API

---

## Migration Guide

### Upgrading from 0.1.0 to 0.1.1

No breaking changes in this release. Simply update the application binary.

Configuration files are compatible between versions.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [egui](https://www.egui.rs/)
- Inspired by [Godot Engine](https://godotengine.org/) and [Godot Manager](https://github.com/eumario/godot-manager)
- Thanks to the open-source community for their amazing tools and libraries