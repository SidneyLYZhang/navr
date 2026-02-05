# Navr

A fast, cross-platform directory navigation tool written in Rust. QuickNav allows you to quickly jump between directories using shortcuts, open file managers, and manage your navigation preferences.

## Features

- 🚀 **Quick Directory Jumping** - Navigate to frequently used directories with short aliases
- 📂 **File Manager Integration** - Open directories in your preferred file manager
- 🔧 **Configurable** - Customize shortcuts, file managers, and behavior
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux
- 🐚 **Shell Integration** - Seamless integration with Bash, Zsh, Fish, and PowerShell
- 📋 **Tab Completions** - Auto-complete shortcuts in your shell
- 📤 **Import/Export** - Backup and share your configuration

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/sidneylyzhang/navr
cd navr

# Build and install
cargo build --release
cargo install --path .
```

### Prerequisites

- Rust 1.70 or later
- For shell integration: Bash, Zsh, Fish, or PowerShell

## Quick Start

```bash
# Add current directory as a shortcut
navr jump --add work

# Jump to a shortcut
navr jump work
# or simply
j work

# Open in file manager
navr open work
# or
jo work

# List all shortcuts
navr jump --list
```

## Commands

### Jump Command

Navigate to directories using shortcuts or paths.

```bash
navr jump [TARGET] [OPTIONS]

Options:
  -l, --list          List all shortcuts
  -a, --add <NAME>    Add current directory as shortcut
  -r, --remove <NAME> Remove a shortcut
```

Examples:
```bash
navr jump work          # Jump to 'work' shortcut
navr jump ~/projects    # Jump to path
navr jump --add dev     # Add current dir as 'dev'
navr jump --remove old  # Remove 'old' shortcut
```

### Open Command

Open directories in file manager.

```bash
navr open [TARGET] [OPTIONS]

Options:
  -w, --with <MANAGER>  Open with specific file manager
```

Examples:
```bash
navr open work          # Open with default file manager
navr open docs --with dolphin  # Open with Dolphin
```

### Quick Mode

Use `-k` or `--quick` for direct opening:

```bash
navr -k work            # Quick open 'work' shortcut
```

### Config Command

Manage configuration.

```bash
navr config <ACTION>

Actions:
  show                    Show current configuration
  edit                    Edit configuration interactively
  set <KEY> <VALUE>       Set configuration value
  get <KEY>               Get configuration value
  reset                   Reset to defaults
  set-file-manager <MAN>  Set default file manager
```

Examples:
```bash
navr config show
navr config set behavior.create_missing true
navr config set-file-manager dolphin
```

### Shell Command

Shell integration and completions.

```bash
navr shell <ACTION>

Actions:
  complete <SHELL>        Generate completion script
  install <SHELL>         Install shell integration
  init <SHELL>            Print init script
```

Examples:
```bash
# Generate completions
navr shell complete bash > /etc/bash_completion.d/navr

# Install shell integration
navr shell install bash
navr shell install zsh
navr shell install fish

# Print init script for manual installation
navr shell init bash
```

### Export/Import

Backup and restore configuration.

```bash
# Export configuration
navr export --format toml --output backup.toml
navr export --format json > backup.json

# Import configuration
navr import backup.toml
navr import backup.json --merge  # Merge with existing
```

## Configuration

Configuration is stored in:

- **Windows**: `%APPDATA%\quicknav\config.toml`
- **macOS**: `~/Library/Application Support/quicknav/config.toml`
- **Linux**: `~/.config/quicknav/config.toml`

### Example Configuration

```toml
default_file_manager = "dolphin"

[shortcuts]
home = "/home/user"
dev = "/home/user/development"
work = "/home/user/work"

[shell]
enabled = true
hook_cd = true
track_history = true
max_history = 1000

[behavior]
confirm_overwrite = true
create_missing = false
follow_symlinks = true
case_sensitive = false
default_to_home = true

[platform.linux]
desktop_env = "kde"
file_manager = "dolphin"
```

## Shell Integration

Navr provides deep shell integration to enhance your workflow.

### Bash

```bash
# Add to ~/.bashrc
eval "$(navr shell init bash)"
```

### Zsh

```bash
# Add to ~/.zshrc
eval "$(navr shell init zsh)"
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
navr shell init fish | source
```

### PowerShell

```powershell
# Add to $PROFILE
navr shell init powershell | Invoke-Expression
```

## Default Shortcuts

Navr comes with sensible defaults for common directories:

| Shortcut | Directory |
|----------|-----------|
| `home`, `~`, `h` | Home directory |
| `desktop`, `desk` | Desktop |
| `docs`, `documents` | Documents |
| `downloads`, `dl` | Downloads |
| `pictures`, `pics` | Pictures |
| `music` | Music |
| `videos` | Videos |
| `config`, `cfg` | Config directory |
| `dev` | ~/dev (if exists) |
| `projects`, `proj` | ~/projects (if exists) |
| `workspace`, `ws` | ~/workspace (if exists) |
| `repos` | ~/repos (if exists) |
| `github`, `gh` | ~/github (if exists) |

## Supported File Managers

### Windows
- Explorer (default)
- Total Commander
- Double Commander
- Files
- OneCommander

### macOS
- Finder (default)
- Path Finder
- ForkLift
- Commander One

### Linux
- xdg-open (default)
- Nautilus (GNOME)
- Dolphin (KDE)
- Thunar (XFCE)
- PCManFM (LXDE/LXQt)
- Nemo (Cinnamon)
- Caja (MATE)
- Ranger (terminal)
- Vifm (terminal)
- Midnight Commander

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Development

### Project Structure

```
quicknav/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # CLI entry point
    ├── config/              # Configuration management
    │   ├── mod.rs
    │   └── defaults.rs
    ├── commands/            # Command implementations
    │   ├── mod.rs
    │   ├── jump.rs
    │   ├── open.rs
    │   ├── config.rs
    │   ├── export.rs
    │   └── import.rs
    ├── platform/            # Platform-specific code
    │   ├── mod.rs
    │   └── file_manager.rs
    └── shell/               # Shell integration
        ├── mod.rs
        ├── completions.rs
        ├── integration.rs
        └── shell_integration.rs
```

## License

This project `Navr` is licensed under the MIT license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [serde](https://github.com/serde-rs/serde) for configuration management
