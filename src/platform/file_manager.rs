//! File manager integration for different platforms

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::config::AppConfig;

/// File manager handler
pub struct FileManager {
    command: String,
}

impl FileManager {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }

    /// Open a path with the configured file manager
    pub fn open(&self, path: &Path, config: &AppConfig) -> Result<()> {
        match self.command.as_str() {
            "explorer" => self.open_windows_explorer(path),
            "open" => self.open_macos_finder(path),
            "finder" => self.open_macos_finder(path),
            "xdg-open" => self.open_linux_xdg(path),
            "nautilus" => self.open_with_args(path, &["nautilus", "--new-window"]),
            "dolphin" => self.open_with_args(path, &["dolphin", "--new-window"]),
            "thunar" => self.open_with_args(path, &["thunar"]),
            "pcmanfm" => self.open_with_args(path, &["pcmanfm"]),
            "nemo" => self.open_with_args(path, &["nemo"]),
            "caja" => self.open_with_args(path, &["caja"]),
            "ranger" => self.open_terminal_file_manager(path, "ranger"),
            "vifm" => self.open_terminal_file_manager(path, "vifm"),
            "mc" => self.open_terminal_file_manager(path, "mc"),
            custom => self.open_custom(path, custom, config),
        }
    }

    fn open_windows_explorer(&self, path: &Path) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            Command::new("explorer")
                .arg(path)
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .context("Failed to open Windows Explorer")?;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            anyhow::bail!("Windows Explorer is only available on Windows")
        }
    }

    fn open_macos_finder(&self, path: &Path) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(path)
                .spawn()
                .context("Failed to open Finder")?;

            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Try using open command anyway (might be available on some systems)
            Command::new("open")
                .arg(path)
                .spawn()
                .context("Failed to open with 'open' command")?;

            Ok(())
        }
    }

    fn open_linux_xdg(&self, path: &Path) -> Result<()> {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .context("Failed to open with xdg-open. Is it installed?")?;

        Ok(())
    }

    fn open_with_args(&self, path: &Path, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            anyhow::bail!("No command specified");
        }

        let mut cmd = Command::new(args[0]);
        cmd.args(&args[1..]).arg(path);

        cmd.spawn()
            .with_context(|| format!("Failed to open with {}", args[0]))?;

        Ok(())
    }

    fn open_terminal_file_manager(&self, path: &Path, fm: &str) -> Result<()> {
        // Detect available terminal emulator
        let terminal = self.detect_terminal()?;

        let mut cmd = Command::new(&terminal);

        // Add terminal-specific arguments
        match terminal.as_str() {
            "gnome-terminal" => {
                cmd.args(&["--window", "--"]).arg(fm).arg(path);
            }
            "konsole" => {
                cmd.args(&["--new-tab", "-e"]).arg(fm).arg(path);
            }
            "xfce4-terminal" => {
                cmd.args(&["--command", &format!("{} '{}'", fm, path.display())]);
            }
            "alacritty" => {
                cmd.args(&["--command", fm, &path.to_string_lossy()]);
            }
            "kitty" => {
                cmd.args(&["--", fm]).arg(path);
            }
            "wezterm" => {
                cmd.args(&["start", "--", fm]).arg(path);
            }
            "xterm" | "rxvt" | "urxvt" => {
                cmd.arg("-e").arg(fm).arg(path);
            }
            _ => {
                // Generic fallback
                cmd.arg("-e").arg(fm).arg(path);
            }
        }

        cmd.spawn()
            .with_context(|| format!("Failed to open {} in terminal", fm))?;

        Ok(())
    }

    fn open_custom(&self, path: &Path, command: &str, _config: &AppConfig) -> Result<()> {
        // Parse command string (may contain arguments)
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.is_empty() {
            anyhow::bail!("Empty custom command");
        }

        let mut cmd = Command::new(parts[0]);
        
        // Add any arguments from the command string
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        
        cmd.arg(path);

        cmd.spawn()
            .with_context(|| format!("Failed to execute custom command: {}", command))?;

        Ok(())
    }

    fn detect_terminal(&self) -> Result<String> {
        let terminals = vec![
            "gnome-terminal",
            "konsole",
            "xfce4-terminal",
            "alacritty",
            "kitty",
            "wezterm",
            "terminator",
            "tilix",
            "xterm",
            "rxvt",
            "urxvt",
        ];

        for term in terminals {
            if which::which(term).is_ok() {
                return Ok(term.to_string());
            }
        }

        // Check environment variables
        if let Ok(term) = std::env::var("TERM") {
            if term != "dumb" && which::which(&term).is_ok() {
                return Ok(term);
            }
        }

        anyhow::bail!("No suitable terminal emulator found")
    }
}

// /// Get the default file manager for the current platform
// pub fn default_file_manager() -> &'static str {
//     #[cfg(target_os = "windows")]
//     {
//         "explorer"
//     }

//     #[cfg(target_os = "macos")]
//     {
//         "open"
//     }

//     #[cfg(target_os = "linux")]
//     {
//         "xdg-open"
//     }

//     #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
//     {
//         "xdg-open"
//     }
// }

// /// Check if a file manager is available
// pub fn is_file_manager_available(fm: &str) -> bool {
//     match fm {
//         "explorer" => cfg!(target_os = "windows"),
//         "open" | "finder" => cfg!(target_os = "macos"),
//         _ => which::which(fm).is_ok(),
//     }
// }

// /// Get available file managers for the current platform
// pub fn available_file_managers() -> Vec<&'static str> {
//     let all = vec![
//         "explorer",
//         "open",
//         "finder",
//         "xdg-open",
//         "nautilus",
//         "dolphin",
//         "thunar",
//         "pcmanfm",
//         "nemo",
//         "caja",
//         "ranger",
//         "vifm",
//         "mc",
//     ];

//     all.into_iter()
//         .filter(|fm| is_file_manager_available(fm))
//         .collect()
// }
