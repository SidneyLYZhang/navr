//! Shell integration binary
//!
//! This binary is used by shells to communicate with navr

use std::env;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "cd" => handle_cd(&args[2..]),
        "complete" => handle_complete(&args[2..]),
        "hook" => handle_hook(&args[2..]),
        "history" => handle_history(&args[2..]),
        "init" => handle_init(&args[2..]),
        _ => print_help(),
    }
}

fn print_help() {
    println!("Navr Shell Integration");
    println!();
    println!("Usage: navr-shell <command> [args...]");
    println!();
    println!("Commands:");
    println!("  cd <target>       - Resolve and output directory path");
    println!("  complete <shell>  - Generate completion script");
    println!("  hook <event>      - Handle shell hook events");
    println!("  history [cmd]     - Manage directory history");
    println!("  init <shell>      - Output initialization script");
}

fn handle_cd(args: &[String]) {
    if args.is_empty() {
        // Default to home directory
        if let Some(home) = dirs::home_dir() {
            println!("{}", home.display());
        }
        return;
    }
    
    let target = &args[0];
    
    // Try direct path first
    let path = PathBuf::from(target);
    if path.is_dir() {
        println!("{}", path.canonicalize().unwrap_or(path).display());
        return;
    }
    
    // Try to load config and resolve shortcut
    if let Ok(config) = load_config() {
        if let Some(shortcut_path) = config.shortcuts.get(target) {
            println!("{}", shortcut_path);
            return;
        }
    }
    
    // Fall back to original target
    println!("{}", target);
}

fn handle_complete(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Shell type required");
        return;
    }
    
    let shell = &args[0];
    let script = match shell.as_str() {
        "bash" => include_str!("integration.rs").split("BASH_INTEGRATION:")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or(""),
        "zsh" => include_str!("integration.rs").split("ZSH_INTEGRATION:")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or(""),
        "fish" => include_str!("integration.rs").split("FISH_INTEGRATION:")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or(""),
        "powershell" => include_str!("integration.rs").split("POWERSHELL_INTEGRATION:")
            .nth(1)
            .and_then(|s| s.split('"').nth(1))
            .unwrap_or(""),
        _ => {
            eprintln!("Unsupported shell: {}", shell);
            return;
        }
    };
    
    println!("{}", script);
}

fn handle_hook(args: &[String]) {
    if args.is_empty() {
        return;
    }
    
    match args[0].as_str() {
        "preexec" => {
            // Called before command execution
            // Could be used to track directory changes
        }
        "precmd" => {
            // Called before prompt display
            // Update history, etc.
        }
        "chpwd" => {
            // Called when directory changes
            if let Ok(config) = load_config() {
                if config.shell.track_history {
                    let _ = add_to_history(&env::current_dir().unwrap_or_default());
                }
            }
        }
        _ => {}
    }
}

fn handle_history(args: &[String]) {
    if args.is_empty() {
        // Show history
        if let Ok(history) = load_history() {
            for (i, entry) in history.iter().enumerate() {
                println!("{}: {}", i + 1, entry);
            }
        }
        return;
    }
    
    match args[0].as_str() {
        "clear" => {
            let _ = clear_history();
        }
        "add" if args.len() > 1 => {
            let path = PathBuf::from(&args[1]);
            let _ = add_to_history(&path);
        }
        _ => {
            eprintln!("Unknown history command: {}", args[0]);
        }
    }
}

fn handle_init(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Shell type required");
        return;
    }
    
    let shell = &args[0];
    let script = generate_init_script(shell);
    println!("{}", script);
}

fn generate_init_script(shell: &str) -> String {
    match shell {
        "bash" => r#"
# Navr initialization for Bash
eval "$(navr shell complete bash)"

# Hook into cd
export NAVR_SHELL=bash
export NAVR_ACTIVE=1

# Directory tracking
_navr_chpwd() {
    if [ -n "$NAVR_ACTIVE" ]; then
        navr-shell hook chpwd 2>/dev/null &
    fi
}

# Install hook
if [[ "${PROMPT_COMMAND:-}" != *"_navr_chpwd"* ]]; then
    PROMPT_COMMAND="_navr_chpwd${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
"#.to_string(),

        "zsh" => r#"
# Navr initialization for Zsh
eval "$(navr shell complete zsh)"

# Hook into cd
export NAVR_SHELL=zsh
export NAVR_ACTIVE=1

# Directory tracking
_navr_chpwd() {
    if [ -n "$NAVR_ACTIVE" ]; then
        navr-shell hook chpwd 2>/dev/null &
    fi
}

# Install hooks
autoload -Uz add-zsh-hook
add-zsh-hook chpwd _navr_chpwd
"#.to_string(),

        "fish" => r#"
# Navr initialization for Fish
eval "$(navr shell complete fish)"

# Hook into cd
export NAVR_SHELL=fish
export NAVR_ACTIVE=1

# Directory tracking
function _navr_chpwd --on-variable PWD
    if test -n "$NAVR_ACTIVE"
        navr-shell hook chpwd 2>/dev/null &
    end
end
"#.to_string(),

        "powershell" => r#"
# Navr initialization for PowerShell
eval "$(navr shell complete powershell)"

# Hook into cd
$env:NAVR_SHELL = "powershell"
$env:NAVR_ACTIVE = "1"

# Directory tracking
$ExecutionContext.SessionState.InvokeCommand.LocationChangedAction = {
    if ($env:NAVR_ACTIVE) {
        Start-Job { navr-shell hook chpwd } | Out-Null
    }
}
"#.to_string(),

        _ => format!("# Unknown shell: {}", shell),
    }
}

// Simple config loading (avoids full config module dependency)
#[derive(serde::Deserialize)]
struct SimpleConfig {
    shortcuts: std::collections::HashMap<String, String>,
    shell: ShellConfig,
}

#[derive(serde::Deserialize)]
struct ShellConfig {
    #[serde(default)]
    track_history: bool,
}

fn load_config() -> Result<SimpleConfig, Box<dyn std::error::Error>> {
    let config_path = dirs::config_dir()
        .ok_or("No config dir")?
        .join("navr")
        .join("config.toml");
    
    let content = std::fs::read_to_string(config_path)?;
    let config: SimpleConfig = toml::from_str(&content)?;
    Ok(config)
}

fn load_history() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let history_path = dirs::data_dir()
        .ok_or("No data dir")?
        .join("navr")
        .join("history.txt");
    
    if !history_path.exists() {
        return Ok(Vec::new());
    }
    
    let content = std::fs::read_to_string(history_path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

fn add_to_history(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = dirs::data_dir()
        .ok_or("No data dir")?
        .join("navr");
    
    std::fs::create_dir_all(&data_dir)?;
    
    let history_path = data_dir.join("history.txt");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(history_path)?;
    
    writeln!(file, "{}", path.display())?;
    Ok(())
}

fn clear_history() -> Result<(), Box<dyn std::error::Error>> {
    let history_path = dirs::data_dir()
        .ok_or("No data dir")?
        .join("navr")
        .join("history.txt");
    
    if history_path.exists() {
        std::fs::remove_file(history_path)?;
    }
    
    Ok(())
}
