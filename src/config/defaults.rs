//! Default configuration values and platform-specific defaults

use super::*;

/// Get platform-specific default shortcuts
pub fn default_shortcuts() -> HashMap<String, String> {
    let mut shortcuts = HashMap::new();

    // Common shortcuts
    if let Some(home) = dirs::home_dir() {
        shortcuts.insert("home".to_string(), home.to_string_lossy().to_string());
        shortcuts.insert("~".to_string(), home.to_string_lossy().to_string());
        shortcuts.insert("h".to_string(), home.to_string_lossy().to_string());
    }

    if let Some(desktop) = dirs::desktop_dir() {
        shortcuts.insert("desktop".to_string(), desktop.to_string_lossy().to_string());
        shortcuts.insert("desk".to_string(), desktop.to_string_lossy().to_string());
    }

    if let Some(documents) = dirs::document_dir() {
        shortcuts.insert("docs".to_string(), documents.to_string_lossy().to_string());
        shortcuts.insert("documents".to_string(), documents.to_string_lossy().to_string());
    }

    if let Some(downloads) = dirs::download_dir() {
        shortcuts.insert("downloads".to_string(), downloads.to_string_lossy().to_string());
        shortcuts.insert("dl".to_string(), downloads.to_string_lossy().to_string());
    }

    if let Some(pictures) = dirs::picture_dir() {
        shortcuts.insert("pictures".to_string(), pictures.to_string_lossy().to_string());
        shortcuts.insert("pics".to_string(), pictures.to_string_lossy().to_string());
    }

    if let Some(music) = dirs::audio_dir() {
        shortcuts.insert("music".to_string(), music.to_string_lossy().to_string());
    }

    if let Some(videos) = dirs::video_dir() {
        shortcuts.insert("videos".to_string(), videos.to_string_lossy().to_string());
    }

    if let Some(config) = dirs::config_dir() {
        shortcuts.insert("config".to_string(), config.to_string_lossy().to_string());
        shortcuts.insert("cfg".to_string(), config.to_string_lossy().to_string());
    }

    // Development shortcuts
    if let Some(home) = dirs::home_dir() {
        let dev = home.join("dev");
        if dev.exists() {
            shortcuts.insert("dev".to_string(), dev.to_string_lossy().to_string());
        }

        let projects = home.join("projects");
        if projects.exists() {
            shortcuts.insert("projects".to_string(), projects.to_string_lossy().to_string());
            shortcuts.insert("proj".to_string(), projects.to_string_lossy().to_string());
        }

        let workspace = home.join("workspace");
        if workspace.exists() {
            shortcuts.insert("workspace".to_string(), workspace.to_string_lossy().to_string());
            shortcuts.insert("ws".to_string(), workspace.to_string_lossy().to_string());
        }

        // Git repositories
        let repos = home.join("repos");
        if repos.exists() {
            shortcuts.insert("repos".to_string(), repos.to_string_lossy().to_string());
        }

        let github = home.join("github");
        if github.exists() {
            shortcuts.insert("github".to_string(), github.to_string_lossy().to_string());
            shortcuts.insert("gh".to_string(), github.to_string_lossy().to_string());
        }
    }

    shortcuts
}

/// Get platform-specific default file manager
pub fn default_file_manager() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "explorer"
    }

    #[cfg(target_os = "macos")]
    {
        "open"
    }

    #[cfg(target_os = "linux")]
    {
        "xdg-open"
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "xdg-open"
    }
}

/// Get list of common file managers for the platform
pub fn common_file_managers() -> Vec<&'static str> {
    #[cfg(target_os = "windows")]
    {
        vec![
            "explorer",
            "totalcmd",
            "doublecmd",
            "files",  // Files (Windows File Manager alternative)
            "onecommander",
        ]
    }

    #[cfg(target_os = "macos")]
    {
        vec![
            "open",           // Default Finder
            "finder",
            "pathfinder",
            "forklift",
            "commanderone",
        ]
    }

    #[cfg(target_os = "linux")]
    {
        vec![
            "xdg-open",       // Default
            "nautilus",       // GNOME Files
            "dolphin",        // KDE
            "thunar",         // XFCE
            "pcmanfm",        // LXDE/LXQt
            "nemo",           // Cinnamon
            "caja",           // MATE
            "ranger",         // Terminal-based
            "vifm",           // Terminal-based
            "mc",             // Midnight Commander
        ]
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        vec!["xdg-open"]
    }
}

/// Detect the current desktop environment on Linux
pub fn detect_desktop_environment() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        use std::env;
        
        // Check XDG_CURRENT_DESKTOP
        if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
            return Some(de.to_lowercase());
        }
        
        // Check DESKTOP_SESSION
        if let Ok(session) = env::var("DESKTOP_SESSION") {
            return Some(session.to_lowercase());
        }
        
        // Check GNOME_DESKTOP_SESSION_ID
        if env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
            return Some("gnome".to_string());
        }
        
        // Check KDE_FULL_SESSION
        if env::var("KDE_FULL_SESSION").is_ok() {
            return Some("kde".to_string());
        }
        
        None
    }

    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Get the best file manager for the current environment
pub fn detect_best_file_manager() -> String {
    #[cfg(target_os = "windows")]
    {
        "explorer".to_string()
    }

    #[cfg(target_os = "macos")]
    {
        "open".to_string()
    }

    #[cfg(target_os = "linux")]
    {
        // Try to detect based on desktop environment
        if let Some(de) = detect_desktop_environment() {
            match de.as_str() {
                "gnome" | "unity" | "pantheon" => {
                    if which::which("nautilus").is_ok() {
                        return "nautilus".to_string();
                    }
                }
                "kde" | "plasma" => {
                    if which::which("dolphin").is_ok() {
                        return "dolphin".to_string();
                    }
                }
                "xfce" => {
                    if which::which("thunar").is_ok() {
                        return "thunar".to_string();
                    }
                }
                "lxde" => {
                    if which::which("pcmanfm").is_ok() {
                        return "pcmanfm".to_string();
                    }
                }
                "cinnamon" => {
                    if which::which("nemo").is_ok() {
                        return "nemo".to_string();
                    }
                }
                "mate" => {
                    if which::which("caja").is_ok() {
                        return "caja".to_string();
                    }
                }
                _ => {}
            }
        }
        
        // Fallback to xdg-open or first available
        for fm in &["xdg-open", "nautilus", "dolphin", "thunar", "pcmanfm"] {
            if which::which(fm).is_ok() {
                return fm.to_string();
            }
        }
        
        "xdg-open".to_string()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "xdg-open".to_string()
    }
}

/// Create a new configuration with smart defaults
pub fn create_default_config() -> AppConfig {
    let mut config = AppConfig::default();
    
    // Add default shortcuts
    config.shortcuts = default_shortcuts();
    
    // Set platform-specific defaults
    #[cfg(target_os = "windows")]
    {
        config.platform.windows.use_windows_terminal = true;
        config.platform.windows.use_powershell_aliases = true;
    }
    
    #[cfg(target_os = "macos")]
    {
        config.platform.macos.use_finder = true;
    }
    
    #[cfg(target_os = "linux")]
    {
        config.platform.linux.desktop_env = detect_desktop_environment();
        config.platform.linux.file_manager = Some(detect_best_file_manager());
    }
    
    config
}
