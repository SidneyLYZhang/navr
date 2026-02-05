//! Import command - Import configuration from various formats

use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::PathBuf;

use crate::config::AppConfig;

pub fn execute(config: &mut AppConfig, input: &str, merge: bool) -> Result<()> {
    let input_path = PathBuf::from(input);
    
    if !input_path.exists() {
        anyhow::bail!("Input file not found: {}", input);
    }

    let content = std::fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read {:?}", input_path))?;

    // Detect format from extension
    let extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("toml")
        .to_lowercase();

    let imported_config = match extension.as_str() {
        "json" => AppConfig::from_json(&content)?,
        "toml" => toml::from_str(&content)
            .with_context(|| "Failed to parse TOML configuration")?,
        "yaml" | "yml" => {
            anyhow::bail!("YAML format not yet implemented. Use json or toml.")
        }
        _ => {
            // Try to detect format from content
            if content.trim().starts_with('{') {
                AppConfig::from_json(&content)?
            } else {
                toml::from_str(&content)
                    .with_context(|| "Failed to parse configuration")?
            }
        }
    };

    if merge {
        config.merge(imported_config);
        println!("{} Configuration merged successfully", "✓".green());
    } else {
        *config = imported_config;
        println!("{} Configuration imported successfully", "✓".green());
    }

    config.save()?;

    println!(
        "  Shortcuts: {}, File managers: {}",
        config.shortcuts.len().to_string().cyan(),
        config.file_managers.len().to_string().cyan()
    );

    Ok(())
}
