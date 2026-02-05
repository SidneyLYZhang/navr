//! Open command - Open directories in file manager

use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::process::Command;

use crate::config::AppConfig;
use crate::platform::file_manager::FileManager;

pub struct OpenCommand {
    target: String,
    file_manager: Option<String>,
}

impl OpenCommand {
    pub fn new(target: String) -> Self {
        Self {
            target,
            file_manager: None,
        }
    }

    pub fn with_manager(target: String, file_manager: Option<String>) -> Self {
        Self {
            target,
            file_manager,
        }
    }

    pub fn execute(&self, config: &AppConfig) -> Result<()> {
        // Resolve target path
        let path = self.resolve_path(config)?;

        // Determine file manager to use
        let fm = self
            .file_manager
            .clone()
            .unwrap_or_else(|| config.get_file_manager());

        // Open the directory
        self.open_directory(&path, &fm, config)?;

        Ok(())
    }

    fn resolve_path(&self, config: &AppConfig) -> Result<PathBuf> {
        // Try to resolve as shortcut first
        if let Some(shortcut_path) = config.get_shortcut(&self.target) {
            return Ok(PathBuf::from(shortcut_path));
        }

        // Expand and resolve as direct path
        let expanded = shellexpand::full(&self.target)?.to_string();
        let path = PathBuf::from(&expanded);

        if path.exists() {
            Ok(path)
        } else if config.behavior.create_missing {
            std::fs::create_dir_all(&path)
                .with_context(|| format!("Failed to create directory: {}", self.target))?;
            println!("{} Created directory: {}", "✓".green(), path.display());
            Ok(path)
        } else {
            anyhow::bail!("Path not found: {}", self.target)
        }
    }

    fn open_directory(&self, path: &PathBuf, fm: &str, config: &AppConfig) -> Result<()> {
        println!(
            "{} Opening {} with {}...",
            "→".blue(),
            path.display().to_string().cyan(),
            fm.yellow()
        );

        let file_manager = FileManager::new(fm);
        file_manager.open(path, config)?;

        Ok(())
    }
}

/// Open a path with the system default file manager
pub fn open_with_default(path: &PathBuf) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        Command::new("explorer")
            .arg(path)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .context("Failed to open file manager")?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .context("Failed to open file manager")?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try xdg-open first
        if which::which("xdg-open").is_ok() {
            Command::new("xdg-open")
                .arg(path)
                .spawn()
                .context("Failed to open file manager")?;
        } else {
            anyhow::bail!("No suitable file manager found. Please install xdg-open.");
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        anyhow::bail!("Unsupported platform");
    }

    Ok(())
}

/// List available file managers
pub fn list_file_managers() -> Vec<(String, bool)> {
    let mut managers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        let candidates = vec![
            ("explorer", true),
            ("totalcmd", false),
            ("doublecmd", false),
            ("files", false),
            ("onecommander", false),
        ];

        for (name, is_default) in candidates {
            let available = which::which(name).is_ok() || is_default;
            managers.push((name.to_string(), available));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = vec![
            ("open", true),
            ("finder", true),
            ("pathfinder", false),
            ("forklift", false),
            ("commanderone", false),
        ];

        for (name, is_default) in candidates {
            let available = which::which(name).is_ok() || is_default;
            managers.push((name.to_string(), available));
        }
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = vec![
            ("xdg-open", true),
            ("nautilus", false),
            ("dolphin", false),
            ("thunar", false),
            ("pcmanfm", false),
            ("nemo", false),
            ("caja", false),
            ("ranger", false),
            ("vifm", false),
            ("mc", false),
        ];

        for (name, is_default) in candidates {
            let available = which::which(name).is_ok() || is_default;
            managers.push((name.to_string(), available));
        }
    }

    managers
}
