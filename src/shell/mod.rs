//! Shell integration and completion generation
use clap::CommandFactory;
use anyhow::{Context, Result};
use clap_complete::{generate, Shell};
use owo_colors::OwoColorize;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::Cli;
use crate::platform::shell_config_path;

pub mod completions;
pub mod integration;

/// Generate shell completion scripts
pub fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    println!(
        "{} Generating {} completions...",
        "→".blue(),
        format!("{:?}", shell).cyan()
    );

    generate(shell, &mut cmd, bin_name, &mut io::stdout());

    println!();
    println!(
        "{} Save this output to your shell's completion directory",
        "ℹ".blue()
    );
    
    match shell {
        Shell::Bash => {
            println!("  Typical location: /etc/bash_completion.d/ or ~/.local/share/bash-completion/completions/");
        }
        Shell::Zsh => {
            println!("  Typical location: /usr/local/share/zsh/site-functions/ or ~/.zsh/completions/");
        }
        Shell::Fish => {
            println!("  Typical location: ~/.config/fish/completions/");
        }
        Shell::PowerShell => {
            println!("  Typical location: $PROFILE directory");
        }
        _ => {}
    }

    Ok(())
}

/// Install shell integration
pub fn install_integration(shell: Shell, path: Option<&str>) -> Result<()> {
    let config_path = match path {
        Some(p) => PathBuf::from(p),
        None => shell_config_path(&format!("{:?}", shell).to_lowercase())?,
    };

    println!(
        "{} Installing {} integration...",
        "→".blue(),
        format!("{:?}", shell).cyan()
    );
    println!("  Target: {}", config_path.display().to_string().dimmed());

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }

    // Generate integration script
    let script = generate_integration_script(shell)?;

    // Check if already installed
    if config_path.exists() {
        let existing = fs::read_to_string(&config_path)?;
        if existing.contains("navr") {
            println!("{} Navr integration already exists", "ℹ".yellow());
            println!("  Run 'navr shell init {:?}' to see the initialization script", shell);
            return Ok(());
        }
    }

    // Append to config file
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_path)
        .with_context(|| format!("Failed to open {:?}", config_path))?;

    use std::io::Write;
    writeln!(file, "\n{}", script)?;

    println!("{} Integration installed successfully!", "✓".green());
    println!();
    println!("{} Please restart your shell or run:", "→".blue());
    match shell {
        Shell::Bash => println!("  source ~/.bashrc"),
        Shell::Zsh => println!("  source ~/.zshrc"),
        Shell::Fish => println!("  source ~/.config/fish/config.fish"),
        Shell::PowerShell => println!("  . $PROFILE"),
        _ => {}
    }

    Ok(())
}

/// Print initialization script for manual installation
pub fn print_init_script(shell: Shell) -> Result<()> {
    let script = generate_integration_script(shell)?;
    println!("{}", script);
    Ok(())
}

/// Generate the appropriate integration script for the shell
fn generate_integration_script(shell: Shell) -> Result<String> {
    match shell {
        Shell::Bash => Ok(integration::BASH_INTEGRATION.to_string()),
        Shell::Zsh => Ok(integration::ZSH_INTEGRATION.to_string()),
        Shell::Fish => Ok(integration::FISH_INTEGRATION.to_string()),
        Shell::PowerShell => Ok(integration::POWERSHELL_INTEGRATION.to_string()),
        Shell::Elvish => Ok(integration::ELVISH_INTEGRATION.to_string()),
        _ => anyhow::bail!("Unsupported shell: {:?}", shell),
    }
}
