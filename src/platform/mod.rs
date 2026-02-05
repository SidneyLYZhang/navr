//! Platform-specific implementations

pub mod file_manager;

use anyhow::Result;
use std::path::PathBuf;


/// Get shell configuration path
pub fn shell_config_path(shell: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    match shell {
        "bash" => Ok(home.join(".bashrc")),
        "zsh" => Ok(home.join(".zshrc")),
        "fish" => {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
            Ok(config_dir.join("fish").join("config.fish"))
        }
        "powershell" => {
            #[cfg(target_os = "windows")]
            {
                let docs = dirs::document_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not find documents directory"))?;
                Ok(docs
                    .join("PowerShell")
                    .join("Microsoft.PowerShell_profile.ps1"))
            }
            #[cfg(not(target_os = "windows"))]
            {
                Ok(home.join(".config").join("powershell").join("Microsoft.PowerShell_profile.ps1"))
            }
        }
        _ => anyhow::bail!("Unsupported shell: {}", shell),
    }
}
