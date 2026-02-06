# Navr Architecture

This document describes the internal architecture of Navr.

## Project Structure

```
navr/
├── Cargo.toml              # Main project manifest
├── README.md               # User documentation
├── QUICKSTART.md           # Quick start guide
├── ARCHITECTURE.md         # This file
├── Makefile                # Build automation
├── install.sh              # Installation script
├── examples/               # Example configurations
│   └── config.example.toml
├── tests/                  # Integration tests
│   └── integration_tests.rs
└── src/
    ├── main.rs             # CLI entry point
    ├── config/             # Configuration management
    │   ├── mod.rs          # Main config module
    │   ├── defaults.rs     # Default values
    │   └── tests.rs        # Unit tests
    ├── commands/           # Command implementations
    │   ├── mod.rs          # Command module
    │   ├── jump.rs         # Jump command
    │   ├── open.rs         # Open command
    │   ├── config.rs       # Config command
    │   ├── export.rs       # Export command
    │   └── import.rs       # Import command
    ├── platform/           # Platform-specific code
    │   ├── mod.rs          # Platform module
    │   └── file_manager.rs # File manager integration
    └── shell/              # Shell integration
        ├── mod.rs          # Shell module
        ├── completions.rs  # Completion generation
        ├── integration.rs  # Shell scripts
        └── shell_integration.rs # Shell helper binary
```

## Module Overview

### `main.rs`

The entry point of the application. Responsibilities:
- CLI argument parsing using `clap`
- Command routing
- Logging initialization
- Error handling

Key structures:
- `Cli`: Main CLI argument structure with global options (`verbose`, `config`, `quick`)
- `Commands`: Enum of all subcommands (Jump, Open, Config, Shell, Export, Import)
- `run()`: Main execution function

Command aliases defined via `visible_alias`:
- `jump` → `j`
- `open` → `o`
- `config` → `cfg`
- `shell` → `sh`
- `export` → `exp`
- `import` → `imp`

### `config/`

Configuration management module.

#### `mod.rs`

Core configuration structures and operations:
- `AppConfig`: Main configuration struct with version, shortcuts, and nested configs
- `ShellConfig`: Shell integration settings (enabled, hook_cd, track_history, max_history, completion_style)
- `BehaviorConfig`: Behavior settings (confirm_overwrite, create_missing, follow_symlinks, case_sensitive, default_to_home)
- `PlatformConfig`: Platform-specific settings (WindowsConfig, MacOSConfig, LinuxConfig)

Key methods:
- `load()`: Load from default location
- `load_from_path()`: Load from specific path
- `save()`: Save to default location
- `get_shortcut()`: Retrieve shortcut path
- `get_file_manager()`: Get platform-appropriate file manager

#### `defaults.rs`

Default values and platform detection:
- `default_shortcuts()`: Generate default shortcuts for common directories
- Platform-specific default detection for home, desktop, documents, downloads, pictures, music, videos, config
- Development shortcuts (dev, projects, workspace, repos, github) - only if directories exist

### `commands/`

Command implementations following a consistent pattern.

Each command module provides:
- A command struct (e.g., `JumpCommand`)
- A `new()` constructor
- An `execute()` method

#### `jump.rs`

Directory navigation command:
- Resolve shortcuts to paths
- List configured shortcuts (grouped by category: System, Development, Custom)
- Add/remove shortcuts
- Fuzzy matching for suggestions
- Path expansion (supports `~` and environment variables)
- Auto-create missing directories (if enabled in config)

#### `open.rs`

File manager integration:
- Open directories in file managers
- Cross-platform file manager support
- Custom file manager selection via `--with`
- Path resolution (shortcuts or direct paths)

#### `config.rs`

Configuration management command:
- Show configuration (formatted display)
- Interactive editing (using `inquire` crate)
- Get/set values
- Reset to defaults
- File manager selection

Subcommands:
- `Show`: Display current configuration
- `Edit`: Interactive configuration editing
- `Set`: Set configuration value by key
- `Get`: Get configuration value by key
- `Reset`: Reset to default configuration
- `SetFileManager`: Set default file manager

#### `export.rs` / `import.rs`

Configuration backup/restore:
- TOML and JSON formats
- Merge or replace on import
- File output or stdout

### `platform/`

Platform-specific abstractions.

#### `file_manager.rs`

File manager integration:
- `FileManager`: Main struct wrapping file manager commands
- Platform-specific open methods
- Support for terminal-based file managers
- Custom command support

### `shell/`

Shell integration and completion.

#### `mod.rs`

Shell integration main module:
- `generate_completions()`: Generate completion scripts for bash, zsh, fish, powershell
- `install_integration()`: Install shell hooks to config files
- `generate_integration_script()`: Output init script

#### `completions.rs`

Dynamic completion support:
- Shortcut completion
- Shell-specific completion generation

#### `integration.rs`

Shell integration scripts as constants:
- `BASH_INTEGRATION`
- `ZSH_INTEGRATION`
- `FISH_INTEGRATION`
- `POWERSHELL_INTEGRATION`
- `ELVISH_INTEGRATION`

Defines aliases:
- `j` → `navr jump`
- `jo` → `navr open`
- `jl` → `navr jump --list`

#### `shell_integration.rs`

Separate binary for shell communication (if needed):
- `cd` resolution
- Hook handling
- History tracking
- Init script generation

## Data Flow

### Jump Command Flow

```
User Input
    ↓
CLI Parsing (clap)
    ↓
JumpCommand::execute()
    ↓
Resolve Target
    ├── --list? → List all shortcuts (grouped by category)
    ├── --add? → Add shortcut to config
    ├── --remove? → Remove shortcut from config
    ├── Shortcut? → Config lookup
    ├── Path? → Expand and validate
    └── Fuzzy search → Suggestions
    ↓
Output Path (for shell to cd)
```

### Open Command Flow

```
User Input
    ↓
CLI Parsing
    ↓
OpenCommand::execute()
    ↓
Resolve Path
    ├── Shortcut? → Config lookup
    └── Path? → Expand and validate
    ↓
Determine File Manager
    ├── Explicit (--with)? → Use specified
    └── Default? → Config or auto-detect
    ↓
FileManager::open()
    ↓
Spawn Process
```

### Configuration Flow

```
Load Config
    ↓
Config Dir Exists?
    ├── Yes → Read TOML → Parse
    └── No → Create Default → Save
    ↓
Use Config
    ↓
Modify? → Save (atomic write)
```

## Key Design Decisions

### 1. Configuration Format

**TOML** chosen for:
- Human-readable
- Comments support
- Clear structure
- Rust ecosystem standard

Configuration path uses `quicknav` directory for backward compatibility.

### 2. Shell Integration Architecture

Two-part design:
1. **Main binary** (`navr`): User-facing commands
2. **Shell scripts**: Embedded in `integration.rs`

Benefits:
- Clean separation of concerns
- No separate shell binary needed
- Easier testing
- Simple installation

### 3. Platform Abstraction

Platform-specific code isolated in `platform/`:
- Easy to add new platforms
- Clear conditional compilation
- Testable abstractions

### 4. Error Handling

Using `anyhow` for:
- Easy error propagation
- Context attachment
- User-friendly messages

Using `thiserror` for:
- Structured error types
- Error enum definitions

### 5. Shell Wrapper Design

The shell integration:
- Defines shell functions for `j`, `jo`, `jl`
- Maintains normal cd behavior
- Minimal overhead
- Tab completion support

### 6. Command Pattern

Each command follows a consistent pattern:
```rust
pub struct CommandName {
    // fields
}

impl CommandName {
    pub fn new(...) -> Self { ... }
    pub fn execute(&self, config: &mut AppConfig) -> Result<()> { ... }
}
```

## Testing Strategy

### Unit Tests

Location: `src/config/tests.rs`

Coverage:
- Config serialization/deserialization
- Shortcut operations
- Value get/set
- Merge behavior

### Integration Tests

Location: `tests/integration_tests.rs`

Coverage:
- CLI argument parsing
- Command execution
- Shell completion generation
- Error handling

### Manual Testing

Scenarios:
- All supported shells (bash, zsh, fish, powershell)
- All platforms (Windows, macOS, Linux)
- Various file managers
- Edge cases (permissions, missing dirs, etc.)

## Performance Considerations

### Startup Time

- Lazy config loading
- Minimal dependencies for core functionality
- Optional features (inquire)

### Config Access

- In-memory caching
- Atomic writes
- Lazy save

### Shell Integration

- Completion caching
- Background history updates
- Minimal shell startup impact

## Security Considerations

### Path Handling

- Path expansion (`~`, env vars) via `shellexpand`
- Symlink following (configurable via `follow_symlinks`)
- Directory traversal prevention

### Config File

- User-owned only
- No sensitive data stored
- Safe serialization via serde

### Shell Integration

- No code injection vectors
- Quote handling
- Safe subprocess spawning

## Dependencies

### Core Dependencies

- `clap` (4.4): CLI parsing with derive macros
- `clap_complete` (4.4): Shell completion generation
- `serde` (1.0): Serialization framework
- `toml` (0.8): TOML parsing/serialization
- `serde_json` (1.0): JSON support
- `anyhow` (1.0): Error handling
- `thiserror` (1.0): Structured errors
- `tracing` (0.1): Logging framework
- `dirs` (5.0): Platform directories
- `owo-colors` (4.0): Terminal colors
- `shellexpand` (3.1): Shell expansion
- `which` (6.0): Command detection

### Optional Dependencies

- `inquire` (0.7): Interactive prompts (feature: `interactive`)

### Platform-specific

- `winapi` (0.3): Windows API
- `windows` (0.52): Windows crate

## Future Extensions

### Planned Features

1. **Frecency-based ranking**: Track usage frequency/recency
2. **Directory history**: Navigate back through history
3. **Fuzzy finder integration**: fzf support
4. **Plugin system**: Custom commands
5. **Remote shortcuts**: SSH path support

### Extension Points

1. **New commands**: Add to `Commands` enum in `main.rs`
2. **New platforms**: Add module to `platform/`
3. **New shells**: Add script to `shell/integration.rs`
4. **New file managers**: Add to `platform/file_manager.rs`

## Contributing

When modifying:

1. **New command**: Add to `commands/`, update `main.rs`
2. **New config option**: Add to `config/mod.rs`, update defaults
3. **New platform**: Add module to `platform/`
4. **New shell**: Add script to `shell/integration.rs`

See `CONTRIBUTING.md` for detailed guidelines.
