//! Export command - Export configuration to various formats

use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::PathBuf;

use crate::config::AppConfig;

pub fn execute(config: &AppConfig, format: &str, output: Option<&str>) -> Result<()> {
    let content = match format.to_lowercase().as_str() {
        "json" => config.to_json()?,
        "toml" => toml::to_string_pretty(config)?,
        _ => anyhow::bail!("Unsupported format: {}. Use json or toml.", format),
    };

    let output_path = match output {
        Some(path) => PathBuf::from(path),
        None => {
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            PathBuf::from(format!("navr_config_{}.{}", timestamp, format))
        }
    };

    std::fs::write(&output_path, &content)
        .with_context(|| format!("Failed to write to {:?}", output_path))?;

    println!(
        "{} Configuration exported to: {}",
        "✓".green(),
        output_path.display().to_string().cyan()
    );
    
    println!(
        "  Format: {}, Size: {} bytes",
        format.yellow(),
        content.len().to_string().dimmed()
    );

    Ok(())
}
