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
- `Cli`: Main CLI argument structure
- `Commands`: Enum of all subcommands
- `run()`: Main execution function

### `config/`

Configuration management module.

#### `mod.rs`

Core configuration structures and operations:
- `AppConfig`: Main configuration struct
- `ShellConfig`: Shell integration settings
- `BehaviorConfig`: Behavior settings
- `PlatformConfig`: Platform-specific settings

Key methods:
- `load()`: Load from default location
- `save()`: Save to default location
- `set_shortcut()`: Add/update shortcut
- `get_shortcut()`: Retrieve shortcut
- `get_file_manager()`: Get platform-appropriate file manager

#### `defaults.rs`

Default values and platform detection:
- `default_shortcuts()`: Generate default shortcuts
- `detect_desktop_environment()`: Detect Linux DE
- `detect_best_file_manager()`: Auto-detect file manager
- `create_default_config()`: Create config with smart defaults

### `commands/`

Command implementations following a consistent pattern.

Each command module provides:
- A command struct (e.g., `JumpCommand`)
- A `new()` constructor
- An `execute()` method

#### `jump.rs`

Directory navigation command:
- Resolve shortcuts to paths
- List configured shortcuts
- Add/remove shortcuts
- Fuzzy matching
- Shell wrapper generation

#### `open.rs`

File manager integration:
- Open directories in file managers
- Cross-platform file manager support
- Terminal-based file manager handling

#### `config.rs`

Configuration management command:
- Show configuration
- Interactive editing
- Get/set values
- Reset to defaults
- File manager selection

#### `export.rs` / `import.rs`

Configuration backup/restore:
- TOML and JSON formats
- Merge or replace on import

### `platform/`

Platform-specific abstractions.

#### `file_manager.rs`

File manager integration:
- `FileManager`: Main struct
- Platform-specific open methods
- Terminal emulator detection
- Custom command support

### `shell/`

Shell integration and completion.

#### `mod.rs`

Shell integration main module:
- `generate_completions()`: Generate completion scripts
- `install_integration()`: Install shell hooks
- `print_init_script()`: Output init script

#### `completions.rs`

Dynamic completion support:
- Shortcut completion
- Completion cache
- Shell-specific scripts

#### `integration.rs`

Shell integration scripts as constants:
- `BASH_INTEGRATION`
- `ZSH_INTEGRATION`
- `FISH_INTEGRATION`
- `POWERSHELL_INTEGRATION`
- `ELVISH_INTEGRATION`

#### `shell_integration.rs`

Separate binary for shell communication:
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
    ↓
Determine File Manager
    ├── Explicit? → Use specified
    └── Default? → Config or auto-detect
    ↓
Platform::FileManager::open()
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

### 2. Shell Integration Architecture

Two-part design:
1. **Main binary** (`navr`): User-facing commands
2. **Shell binary** (`navr-shell`): Shell communication

Benefits:
- Clean separation of concerns
- Shell can call helper without full CLI overhead
- Easier testing

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

### 5. Shell Wrapper Design

The `cd` wrapper:
- Tries direct path first
- Falls back to shortcut resolution
- Maintains normal cd behavior
- Minimal overhead

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
- All supported shells
- All platforms
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

- Path expansion (`~`, env vars)
- Symlink following (configurable)
- Directory traversal prevention

### Config File

- User-owned only
- No sensitive data stored
- Safe serialization

### Shell Integration

- No code injection vectors
- Quote handling
- Safe subprocess spawning

## Future Extensions

### Planned Features

1. **Frecency-based ranking**: Track usage frequency/recency
2. **Directory history**: Navigate back through history
3. **Fuzzy finder integration**: fzf support
4. **Plugin system**: Custom commands
5. **Remote shortcuts**: SSH path support

### Extension Points

1. **New commands**: Add to `Commands` enum
2. **New platforms**: Add to `platform/`
3. **New shells**: Add to `shell/integration.rs`
4. **New file managers**: Add to `platform/file_manager.rs`

## Contributing

When modifying:

1. **New command**: Add to `commands/`, update `main.rs`
2. **New config option**: Add to `config/mod.rs`, update defaults
3. **New platform**: Add module to `platform/`
4. **New shell**: Add script to `shell/integration.rs`

See `CONTRIBUTING.md` for detailed guidelines.
