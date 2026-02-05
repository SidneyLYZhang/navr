//! Configuration management for Navr
//!
//! Handles loading, saving, and modifying application configuration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod defaults;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Version of the configuration format
    #[serde(default = "default_version")]
    pub version: String,

    /// Default file manager to use
    #[serde(default)]
    pub default_file_manager: Option<String>,

    /// Directory shortcuts (alias -> path)
    #[serde(default)]
    pub shortcuts: HashMap<String, String>,

    /// Shell integration settings
    #[serde(default)]
    pub shell: ShellConfig,

    /// Behavior settings
    #[serde(default)]
    pub behavior: BehaviorConfig,

    /// Platform-specific settings
    #[serde(default)]
    pub platform: PlatformConfig,

    /// Custom file managers per platform
    #[serde(default)]
    pub file_managers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellConfig {
    /// Enable shell integration
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Auto-completion style
    #[serde(default = "default_completion_style")]
    pub completion_style: String,

    /// Hook into cd command
    #[serde(default = "default_true")]
    pub hook_cd: bool,

    /// Track directory history
    #[serde(default = "default_true")]
    pub track_history: bool,

    /// Maximum history entries
    #[serde(default = "default_max_history")]
    pub max_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BehaviorConfig {
    /// Confirm before overwriting shortcuts
    #[serde(default = "default_true")]
    pub confirm_overwrite: bool,

    /// Create missing directories
    #[serde(default = "default_false")]
    pub create_missing: bool,

    /// Follow symbolic links
    #[serde(default = "default_true")]
    pub follow_symlinks: bool,

    /// Case-sensitive shortcut matching
    #[serde(default = "default_false")]
    pub case_sensitive: bool,

    /// Default to home directory if no target specified
    #[serde(default = "default_true")]
    pub default_to_home: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformConfig {
    /// Windows-specific settings
    #[serde(default)]
    pub windows: WindowsConfig,

    /// macOS-specific settings
    #[serde(default)]
    pub macos: MacOSConfig,

    /// Linux-specific settings
    #[serde(default)]
    pub linux: LinuxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowsConfig {
    /// Use Windows Terminal
    #[serde(default = "default_true")]
    pub use_windows_terminal: bool,

    /// Use PowerShell aliases
    #[serde(default = "default_true")]
    pub use_powershell_aliases: bool,

    /// Preferred file manager
    #[serde(default)]
    pub file_manager: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacOSConfig {
    /// Use Finder integration
    #[serde(default = "default_true")]
    pub use_finder: bool,

    /// Use iTerm2 if available
    #[serde(default = "default_false")]
    pub prefer_iterm2: bool,

    /// Preferred file manager
    #[serde(default)]
    pub file_manager: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinuxConfig {
    /// Preferred terminal
    #[serde(default)]
    pub terminal: Option<String>,

    /// Desktop environment
    #[serde(default)]
    pub desktop_env: Option<String>,

    /// Preferred file manager
    #[serde(default)]
    pub file_manager: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            default_file_manager: None,
            shortcuts: HashMap::new(),
            shell: ShellConfig::default(),
            behavior: BehaviorConfig::default(),
            platform: PlatformConfig::default(),
            file_managers: HashMap::new(),
        }
    }
}

impl AppConfig {
    /// Load configuration from default location
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            Self::load_from_path(&config_path)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Load configuration from specific path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path.as_ref()))?;
        
        let config: AppConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config from {:?}", path.as_ref()))?;
        
        Ok(config)
    }

    /// Save configuration to default location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to {:?}", config_path))?;
        
        Ok(())
    }

    /// Get default configuration path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?;
        Ok(config_dir.join("navr").join("config.toml"))
    }

    /// Add or update a shortcut
    pub fn set_shortcut(&mut self, name: &str, path: &str) -> Result<()> {
        let expanded = shellexpand::full(path)?.to_string();
        let canonical = std::fs::canonicalize(&expanded)
            .unwrap_or_else(|_| PathBuf::from(&expanded));
        
        self.shortcuts.insert(name.to_string(), canonical.to_string_lossy().to_string());
        self.save()?;
        Ok(())
    }

    /// Remove a shortcut
    pub fn remove_shortcut(&mut self, name: &str) -> Result<bool> {
        let removed = self.shortcuts.remove(name).is_some();
        if removed {
            self.save()?;
        }
        Ok(removed)
    }

    /// Get shortcut path
    pub fn get_shortcut(&self, name: &str) -> Option<&String> {
        if self.behavior.case_sensitive {
            self.shortcuts.get(name)
        } else {
            self.shortcuts.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(name))
                .map(|(_, v)| v)
        }
    }

    /// Get default file manager for current platform
    pub fn get_file_manager(&self) -> String {
        // Check explicit setting first
        if let Some(fm) = &self.default_file_manager {
            return fm.clone();
        }

        // Check platform-specific setting
        #[cfg(target_os = "windows")]
        {
            self.platform.windows.file_manager.clone()
                .unwrap_or_else(|| "explorer".to_string())
        }

        #[cfg(target_os = "macos")]
        {
            self.platform.macos.file_manager.clone()
                .unwrap_or_else(|| "open".to_string())
        }

        #[cfg(target_os = "linux")]
        {
            self.platform.linux.file_manager.clone()
                .unwrap_or_else(|| {
                    // Try to detect common file managers
                    for fm in &["xdg-open", "nautilus", "dolphin", "thunar", "pcmanfm"] {
                        if which::which(fm).is_ok() {
                            return fm.to_string();
                        }
                    }
                    "xdg-open".to_string()
                })
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            "xdg-open".to_string()
        }
    }

    /// Set configuration value by key
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "default_file_manager" => self.default_file_manager = Some(value.to_string()),
            "shell.enabled" => self.shell.enabled = value.parse()?,
            "shell.hook_cd" => self.shell.hook_cd = value.parse()?,
            "shell.track_history" => self.shell.track_history = value.parse()?,
            "shell.max_history" => self.shell.max_history = value.parse()?,
            "behavior.confirm_overwrite" => self.behavior.confirm_overwrite = value.parse()?,
            "behavior.create_missing" => self.behavior.create_missing = value.parse()?,
            "behavior.follow_symlinks" => self.behavior.follow_symlinks = value.parse()?,
            "behavior.case_sensitive" => self.behavior.case_sensitive = value.parse()?,
            "behavior.default_to_home" => self.behavior.default_to_home = value.parse()?,
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
        self.save()?;
        Ok(())
    }

    /// Get configuration value by key
    pub fn get_value(&self, key: &str) -> Result<String> {
        match key {
            "default_file_manager" => Ok(self.default_file_manager.clone().unwrap_or_default()),
            "shell.enabled" => Ok(self.shell.enabled.to_string()),
            "shell.hook_cd" => Ok(self.shell.hook_cd.to_string()),
            "shell.track_history" => Ok(self.shell.track_history.to_string()),
            "shell.max_history" => Ok(self.shell.max_history.to_string()),
            "behavior.confirm_overwrite" => Ok(self.behavior.confirm_overwrite.to_string()),
            "behavior.create_missing" => Ok(self.behavior.create_missing.to_string()),
            "behavior.follow_symlinks" => Ok(self.behavior.follow_symlinks.to_string()),
            "behavior.case_sensitive" => Ok(self.behavior.case_sensitive.to_string()),
            "behavior.default_to_home" => Ok(self.behavior.default_to_home.to_string()),
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }
    }

    /// Merge with another configuration
    pub fn merge(&mut self, other: AppConfig) {
        // Merge shortcuts
        for (k, v) in other.shortcuts {
            self.shortcuts.entry(k).or_insert(v);
        }

        // Merge file managers
        for (k, v) in other.file_managers {
            self.file_managers.entry(k).or_insert(v);
        }

        // Override other settings if they're not default
        if other.default_file_manager.is_some() {
            self.default_file_manager = other.default_file_manager;
        }
    }

    /// Export to JSON format
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Import from JSON format
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
}

// Helper functions for serde defaults
fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_completion_style() -> String {
    "fuzzy".to_string()
}

fn default_max_history() -> usize {
    1000
}

#[cfg(test)]
mod tests;
