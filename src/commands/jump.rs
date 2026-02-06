//! Jump command - Navigate to directories using shortcuts

use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::env;
use std::path::PathBuf;

use crate::config::AppConfig;

pub struct JumpCommand {
    target: Option<String>,
    list: bool,
    add: Option<String>,
    remove: Option<String>,
}

impl JumpCommand {
    pub fn new(
        target: Option<String>,
        list: bool,
        add: Option<String>,
        remove: Option<String>,
    ) -> Self {
        Self {
            target,
            list,
            add,
            remove,
        }
    }

    pub fn execute(&self, config: &mut AppConfig) -> Result<()> {
        // Handle list flag
        if self.list {
            return self.list_shortcuts(config);
        }

        // Handle add flag
        if let Some(name) = &self.add {
            return self.add_shortcut(config, name);
        }

        // Handle remove flag
        if let Some(name) = &self.remove {
            return self.remove_shortcut(config, name);
        }

        // Handle jump to target
        match &self.target {
            Some(target) => self.jump_to(config, target),
            None => {
                // No target - jump to home or list shortcuts
                if config.behavior.default_to_home {
                    if let Some(home) = dirs::home_dir() {
                        self.output_path(&home);
                        Ok(())
                    } else {
                        self.list_shortcuts(config)
                    }
                } else {
                    self.list_shortcuts(config)
                }
            }
        }
    }

    fn jump_to(&self, config: &AppConfig, target: &str) -> Result<()> {
        // First, try to resolve as shortcut
        if let Some(path) = config.get_shortcut(target) {
            self.output_path(&PathBuf::from(path));
            return Ok(());
        }

        // Try as direct path
        let expanded = shellexpand::full(target)?.to_string();
        let path = PathBuf::from(&expanded);

        if path.exists() {
            if path.is_dir() {
                self.output_path(&path);
                Ok(())
            } else {
                anyhow::bail!("'{}' is a file, not a directory", target)
            }
        } else if config.behavior.create_missing {
            // Create the directory if it doesn't exist
            std::fs::create_dir_all(&path)
                .with_context(|| format!("Failed to create directory: {}", target))?;
            println!("{} Created directory: {}", "✓".green(), path.display());
            self.output_path(&path);
            Ok(())
        } else {
            // Try fuzzy matching on shortcuts
            let matches = self.fuzzy_find_shortcuts(config, target);
            if !matches.is_empty() {
                println!("{} Did you mean:", "?".yellow());
                for (name, path) in matches.iter().take(5) {
                    println!("  {} -> {}", name.cyan(), path.dimmed());
                }
            }
            anyhow::bail!("Directory not found: {}", target)
        }
    }

    fn list_shortcuts(&self, config: &AppConfig) -> Result<()> {
        if config.shortcuts.is_empty() {
            println!("{} No shortcuts configured", "ℹ".blue());
            println!("Use 'navr jump --add <name>' to add the current directory");
            return Ok(());
        }

        println!("{}", "Configured Shortcuts:".bold().underline());
        println!();

        // Group shortcuts by category
        let mut system = Vec::new();
        let mut dev = Vec::new();
        let mut custom = Vec::new();

        for (name, path) in &config.shortcuts {
            let entry = (name.as_str(), path.as_str());
            match name.as_str() {
                "home" | "~" | "h" | "desktop" | "desk" | "docs" | "documents" 
                | "downloads" | "dl" | "pictures" | "pics" | "music" | "videos" 
                | "config" | "cfg" => system.push(entry),
                "dev" | "projects" | "proj" | "workspace" | "ws" | "repos" 
                | "github" | "gh" => dev.push(entry),
                _ => custom.push(entry),
            }
        }

        // Print system shortcuts
        if !system.is_empty() {
            println!("{}", "System:".bold());
            self.print_shortcut_list(&system);
        }

        // Print dev shortcuts
        if !dev.is_empty() {
            println!("{}", "Development:".bold());
            self.print_shortcut_list(&dev);
        }

        // Print custom shortcuts
        if !custom.is_empty() {
            println!("{}", "Custom:".bold());
            self.print_shortcut_list(&custom);
        }

        println!();
        println!(
            "{} Use 'navr jump <name>' to navigate",
            "→".dimmed()
        );

        Ok(())
    }

    fn print_shortcut_list(&self, shortcuts: &[(&str, &str)]) {
        let max_len = shortcuts.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
        
        for (name, path) in shortcuts {
            let padding = " ".repeat(max_len - name.len());
            println!(
                "  {}{}  {} {}",
                name.cyan().bold(),
                padding,
                "→".dimmed(),
                path.dimmed()
            );
        }
        println!();
    }

    fn add_shortcut(&self, config: &mut AppConfig, name: &str) -> Result<()> {
        let current_dir = env::current_dir()
            .context("Failed to get current directory")?;

        // Check if shortcut already exists
        if config.shortcuts.contains_key(name) && config.behavior.confirm_overwrite {
            print!(
                "{} Shortcut '{}' already exists. Overwrite? [y/N] ",
                "?".yellow(),
                name
            );
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("{} Cancelled", "✗".red());
                return Ok(());
            }
        }

        config.set_shortcut(name, current_dir.to_str().unwrap())?;
        
        println!(
            "{} Added shortcut: {} → {}",
            "✓".green(),
            name.cyan(),
            current_dir.display()
        );

        Ok(())
    }

    fn remove_shortcut(&self, config: &mut AppConfig, name: &str) -> Result<()> {
        if config.remove_shortcut(name)? {
            println!("{} Removed shortcut: {}", "✓".green(), name.cyan());
        } else {
            println!("{} Shortcut '{}' not found", "✗".red(), name);
        }
        Ok(())
    }

    fn fuzzy_find_shortcuts<'a>(&self, config: &'a AppConfig, target: &str) -> Vec<(&'a String, &'a String)> {
        let target_lower = target.to_lowercase();
        
        config
            .shortcuts
            .iter()
            .filter(|(name, _)| {
                let name_lower = name.to_lowercase();
                name_lower.contains(&target_lower) || 
                target_lower.contains(&name_lower)
            })
            .collect()
    }

    fn output_path(&self, path: &PathBuf) {
        // Output the path for shell integration to capture
        // The shell wrapper will use this to actually change directory
        // Use a special marker to indicate this is a jump request
        
        // On Windows, handle path canonicalization and formatting
        let path_str = if cfg!(windows) {
            // For Windows, use absolute path without canonicalize to avoid \\?\ prefix
            let absolute_path = if path.is_absolute() {
                path.clone()
            } else {
                // Convert relative path to absolute
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(path)
            };
            
            // Convert to string and normalize separators
            absolute_path.to_string_lossy().replace('/', "\\")
        } else {
            // For Unix-like systems, use canonicalize
            path.canonicalize().unwrap_or(path.clone()).to_string_lossy().to_string()
        };
        
        println!("NAVR_JUMP:{}", path_str);
    }
}
