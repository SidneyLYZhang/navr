//! Config command - Manage application configuration

use anyhow::Result;
use owo_colors::OwoColorize;
use inquire::{Confirm, Select};
use clap::Subcommand;

use crate::config::AppConfig;
use crate::commands::open::list_file_managers;
use crate::config::defaults::create_default_config;

pub struct ConfigCommand {
    action: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show current configuration
    Show,
    /// Edit configuration interactively
    Edit,
    /// Set configuration value
    Set {
        /// Configuration key (e.g., 'default_file_manager')
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Reset configuration to defaults
    Reset,
    /// Set default file manager
    SetFileManager {
        /// File manager command or 'auto' for system default
        manager: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ShellSubCommand {
    /// Generate shell completion script
    Complete {
        /// Shell type (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Install shell integration
    Install {
        /// Shell type
        #[arg(value_enum)]
        shell: clap_complete::Shell,

        /// Installation path
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Generate initialization script for shell integration
    Init {
        /// Shell type
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl ConfigCommand {
    pub fn new(action: ConfigSubCommand) -> Self {
        Self { action }
    }

    pub fn execute(&self, config: &mut AppConfig) -> Result<()> {
        match &self.action {
            ConfigSubCommand::Show => self.show_config(config),
            ConfigSubCommand::Edit => self.edit_interactive(config),
            ConfigSubCommand::Set { key, value } => self.set_value(config, key, value),
            ConfigSubCommand::Get { key } => self.get_value(config, key),
            ConfigSubCommand::Reset => self.reset_config(config),
            ConfigSubCommand::SetFileManager { manager } => {
                self.set_file_manager(config, manager)
            }
        }
    }

    fn show_config(&self, config: &AppConfig) -> Result<()> {
        println!("{}", "Current Configuration:".bold().underline());
        println!();

        // General settings
        println!("{}", "General:".bold());
        println!(
            "  {}: {}",
            "Version".cyan(),
            config.version.dimmed()
        );
        println!(
            "  {}: {}",
            "Default File Manager".cyan(),
            config
                .default_file_manager
                .as_deref()
                .unwrap_or("auto-detect")
                .yellow()
        );
        println!();

        // Shortcuts
        println!("{}", "Shortcuts:".bold());
        println!("  {} shortcuts configured", config.shortcuts.len().to_string().cyan());
        if !config.shortcuts.is_empty() {
            let preview: Vec<_> = config.shortcuts.iter().take(5).collect();
            for (name, path) in preview {
                println!("  {} → {}", name.cyan(), path.dimmed());
            }
            if config.shortcuts.len() > 5 {
                println!("  ... and {} more", config.shortcuts.len() - 5);
            }
        }
        println!();

        // Shell settings
        println!("{}", "Shell Integration:".bold());
        println!("  {}: {}", "Enabled".cyan(), format_bool(config.shell.enabled));
        println!("  {}: {}", "Hook cd".cyan(), format_bool(config.shell.hook_cd));
        println!(
            "  {}: {}",
            "Track History".cyan(),
            format_bool(config.shell.track_history)
        );
        println!(
            "  {}: {}",
            "Max History".cyan(),
            config.shell.max_history.to_string().yellow()
        );
        println!();

        // Behavior settings
        println!("{}", "Behavior:".bold());
        println!(
            "  {}: {}",
            "Confirm Overwrite".cyan(),
            format_bool(config.behavior.confirm_overwrite)
        );
        println!(
            "  {}: {}",
            "Create Missing".cyan(),
            format_bool(config.behavior.create_missing)
        );
        println!(
            "  {}: {}",
            "Follow Symlinks".cyan(),
            format_bool(config.behavior.follow_symlinks)
        );
        println!(
            "  {}: {}",
            "Case Sensitive".cyan(),
            format_bool(config.behavior.case_sensitive)
        );
        println!();

        // Platform settings
        #[cfg(target_os = "windows")]
        {
            println!("{}", "Windows Settings:".bold());
            println!(
                "  {}: {}",
                "Use Windows Terminal".cyan(),
                format_bool(config.platform.windows.use_windows_terminal)
            );
            println!(
                "  {}: {}",
                "PowerShell Aliases".cyan(),
                format_bool(config.platform.windows.use_powershell_aliases)
            );
        }

        #[cfg(target_os = "macos")]
        {
            println!("{}", "macOS Settings:".bold());
            println!(
                "  {}: {}",
                "Use Finder".cyan(),
                format_bool(config.platform.macos.use_finder)
            );
            println!(
                "  {}: {}",
                "Prefer iTerm2".cyan(),
                format_bool(config.platform.macos.prefer_iterm2)
            );
        }

        #[cfg(target_os = "linux")]
        {
            println!("{}", "Linux Settings:".bold());
            println!(
                "  {}: {}",
                "Desktop Environment".cyan(),
                config
                    .platform
                    .linux
                    .desktop_env
                    .as_deref()
                    .unwrap_or("auto-detect")
                    .yellow()
            );
            println!(
                "  {}: {}",
                "File Manager".cyan(),
                config
                    .platform
                    .linux
                    .file_manager
                    .as_deref()
                    .unwrap_or("auto-detect")
                    .yellow()
            );
        }

        println!();
        println!(
            "{} Config file: {}",
            "ℹ".blue(),
            AppConfig::config_path()?.display().to_string().dimmed()
        );

        Ok(())
    }

    fn edit_interactive(&self, config: &mut AppConfig) -> Result<()> {
        println!("{}", "Interactive Configuration Editor".bold().underline());
        println!();

        // Edit shell settings
        let shell_enabled = Confirm::new("Enable shell integration?")
            .with_default(config.shell.enabled)
            .prompt()?;
        config.shell.enabled = shell_enabled;

        let hook_cd = Confirm::new("Hook into cd command?")
            .with_default(config.shell.hook_cd)
            .prompt()?;
        config.shell.hook_cd = hook_cd;

        let track_history = Confirm::new("Track directory history?")
            .with_default(config.shell.track_history)
            .prompt()?;
        config.shell.track_history = track_history;

        // Edit behavior settings
        let confirm_overwrite = Confirm::new("Confirm before overwriting shortcuts?")
            .with_default(config.behavior.confirm_overwrite)
            .prompt()?;
        config.behavior.confirm_overwrite = confirm_overwrite;

        let create_missing = Confirm::new("Create missing directories?")
            .with_default(config.behavior.create_missing)
            .prompt()?;
        config.behavior.create_missing = create_missing;

        // Select file manager
        let managers = list_file_managers();
        let manager_names: Vec<_> = managers.iter().map(|(n, _)| n.as_str()).collect();
        
        let current_fm = config.default_file_manager.as_deref().unwrap_or("auto");
        let selection = Select::new("Select default file manager:", manager_names)
            .with_starting_cursor(
                managers
                    .iter()
                    .position(|(n, _)| n == current_fm)
                    .unwrap_or(0),
            )
            .prompt()?;

        config.default_file_manager = Some(selection.to_string());

        // Save changes
        config.save()?;
        
        println!();
        println!("{} Configuration saved!", "✓".green());

        Ok(())
    }

    fn set_value(&self, config: &mut AppConfig, key: &str, value: &str) -> Result<()> {
        config.set_value(key, value)?;
        println!(
            "{} Set {} = {}",
            "✓".green(),
            key.cyan(),
            value.yellow()
        );
        Ok(())
    }

    fn get_value(&self, config: &AppConfig, key: &str) -> Result<()> {
        let value = config.get_value(key)?;
        println!("{} = {}", key.cyan(), value.yellow());
        Ok(())
    }

    fn reset_config(&self, config: &mut AppConfig) -> Result<()> {
        let confirm = Confirm::new(
            "Are you sure you want to reset all configuration to defaults?"
        )
        .with_default(false)
        .prompt()?;

        if confirm {
            *config = create_default_config();
            config.save()?;
            println!("{} Configuration reset to defaults", "✓".green());
        } else {
            println!("{} Cancelled", "✗".red());
        }

        Ok(())
    }

    fn set_file_manager(&self, config: &mut AppConfig, manager: &str) -> Result<()> {
        let manager = if manager == "auto" {
            None
        } else {
            Some(manager.to_string())
        };

        config.default_file_manager = manager.clone();
        config.save()?;

        println!(
            "{} Default file manager set to: {}",
            "✓".green(),
            manager.as_deref().unwrap_or("auto-detect").cyan()
        );

        Ok(())
    }
}

fn format_bool(value: bool) -> String {
    if value {
        "true".green().to_string()
    } else {
        "false".red().to_string()
    }
}
