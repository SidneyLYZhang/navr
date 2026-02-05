# Navr Quick Start Guide

Get up and running with Navr in 5 minutes!

## Installation

### Option 1: Using the install script (Recommended)

```bash
curl -sSL https://raw.githubusercontent.com/sidneylyzhang/navr/main/install.sh | bash
```

### Option 2: Build from source

```bash
git clone https://github.com/sidneylyzhang/navr
cd navr
cargo build --release
sudo cp target/release/navr /usr/local/bin/
```

### Option 3: Using Cargo

```bash
cargo install navr
```

## Initial Setup

### 1. Add shell integration

Add the following to your shell configuration file:

**Bash** (`~/.bashrc`):
```bash
eval "$(navr shell init bash)"
```

**Zsh** (`~/.zshrc`):
```bash
eval "$(navr shell init zsh)"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
navr shell init fish | source
```

**PowerShell** (`$PROFILE`):
```powershell
navr shell init powershell | Invoke-Expression
```

### 2. Reload your shell

```bash
source ~/.bashrc  # or .zshrc, etc.
```

## Basic Usage

### Add shortcuts

```bash
# Navigate to a directory you use often
cd ~/projects/my-awesome-project

# Add it as a shortcut
navr jump --add awesome
# or simply
j --add awesome
```

### Jump to shortcuts

```bash
# Jump using the shortcut
j awesome

# Or use the full command
navr jump awesome
```

### List all shortcuts

```bash
j --list
# or
jl  # if you have the alias set up
```

### Open in file manager

```bash
# Open the shortcut in your default file manager
jo awesome

# Or specify a file manager
navr open awesome --with dolphin
```

## Quick Mode

Use the `-k` or `--quick` flag for direct opening:

```bash
navr -k awesome
```

This is useful for scripting and quick access.

## Configuration

### View configuration

```bash
navr config show
```

### Set default file manager

```bash
navr config set-file-manager dolphin
```

### Edit configuration interactively

```bash
navr config edit
```

### Manual configuration

Edit the config file directly:

- **Linux/macOS**: `~/.config/quicknav/config.toml`
- **Windows**: `%APPDATA%\quicknav\config.toml`

Example:
```toml
default_file_manager = "dolphin"

[shortcuts]
home = "/home/username"
work = "/home/username/work"
projects = "/home/username/projects"
```

## Shell Shortcuts

After installing shell integration, you get these aliases:

| Alias | Command | Description |
|-------|---------|-------------|
| `j` | `quicknav jump` | Jump to shortcut |
| `jo` | `quicknav open` | Open in file manager |
| `jl` | `quicknav jump --list` | List shortcuts |
| `jc` | `quicknav config show` | Show config |

## Tab Completion

QuickNav provides tab completion for shortcuts:

```bash
j wo<TAB>    # Completes to 'work' if you have that shortcut
j pro<TAB>   # Completes to 'projects'
```

## Tips & Tricks

### 1. Fuzzy matching

Shortcuts are matched fuzzily by default:

```bash
j wo      # Matches 'work', 'workspace', etc.
```

### 2. Path expansion

You can use paths with shortcuts:

```bash
j awesome/src
```

### 3. Create directories on the fly

Enable in config:
```toml
[behavior]
create_missing = true
```

Then:
```bash
j newproject  # Creates the directory if it doesn't exist
```

### 4. Import/Export configuration

Backup your config:
```bash
navr export --format toml --output backup.toml
```

Restore on another machine:
```bash
navr import backup.toml
```

### 5. Use with other tools

Combine with `ls`:
```bash
j awesome && ls -la
```

Use in scripts:
```bash
#!/bin/bash
PROJECT_DIR=$(navr jump awesome)
cd "$PROJECT_DIR"
# Do something...
```

## Troubleshooting

### Command not found

Make sure the binary is in your PATH:
```bash
which navr
```

If not, add to your PATH:
```bash
export PATH="$PATH:/path/to/navr"
```

### Shell integration not working

1. Check that you reloaded your shell config
2. Verify the init command works:
   ```bash
   navr shell init bash
   ```
3. Check for errors in your shell config

### Shortcuts not resolving

1. Check if shortcut exists:
   ```bash
   j --list
   ```
2. Verify the path is valid:
   ```bash
   navr config get shortcuts.<name>
   ```

## Next Steps

- Read the full [README.md](README.md)
- Explore all commands with `navr --help`
- Check out example configurations in `examples/`

## Getting Help

- Open an issue: https://github.com/sidneylyzhang/navr/issues
- Read the docs: https://github.com/sidneylyzhang/navr/wiki
