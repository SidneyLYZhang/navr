# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial changelog file

## [0.1.6] - 2025-02-05

### Added
- Support for aarch64-apple-darwin target (Apple Silicon macOS)
- Platform-specific strip commands for aarch64 architectures

### Changed
- Improved aarch64 cross-compilation configuration
- Enhanced build workflow with better cross-platform support

## [0.1.4] - 2025-02-05

### Added
- Shell integration support for Bash, Zsh, Fish, and PowerShell
- Tab completion functionality for shortcuts
- Import/export capabilities for configuration backup and sharing
- Interactive prompts feature (optional)
- Comprehensive logging with tracing framework

### Changed
- Improved error handling with thiserror and anyhow crates
- Enhanced configuration management with confy
- Updated dependencies to latest stable versions

### Fixed
- Cross-platform compatibility improvements
- Path expansion handling with shellexpand
- File manager integration stability

## [0.1.0] - Initial Release

### Added
- Core directory navigation functionality
- Jump command for quick directory switching
- Open command for file manager integration
- Configuration system with TOML support
- Cross-platform support for Windows, macOS, and Linux
- Basic shortcut management (add, remove, list)
- Colorized terminal output
- License and documentation