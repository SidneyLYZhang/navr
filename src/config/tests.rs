//! Unit tests for configuration module

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
        assert!(config.shortcuts.is_empty());
        assert!(config.default_file_manager.is_none());
    }

    #[test]
    fn test_shortcut_operations() {
        let mut config = AppConfig::default();
        
        // Add shortcut
        config.set_shortcut("test", "/tmp/test").unwrap();
        assert!(config.shortcuts.contains_key("test"));
        
        // Get shortcut
        let path = config.get_shortcut("test");
        assert!(path.is_some());
        assert!(path.unwrap().contains("test"));
        
        // Remove shortcut
        assert!(config.remove_shortcut("test").unwrap());
        assert!(!config.shortcuts.contains_key("test"));
        
        // Remove non-existent
        assert!(!config.remove_shortcut("nonexistent").unwrap());
    }

    #[test]
    fn test_case_insensitive_matching() {
        let mut config = AppConfig::default();
        config.behavior.case_sensitive = false;
        config.set_shortcut("Test", "/tmp/test").unwrap();
        
        assert!(config.get_shortcut("test").is_some());
        assert!(config.get_shortcut("TEST").is_some());
        assert!(config.get_shortcut("Test").is_some());
    }

    #[test]
    fn test_case_sensitive_matching() {
        let mut config = AppConfig::default();
        config.behavior.case_sensitive = true;
        config.set_shortcut("Test", "/tmp/test").unwrap();
        
        assert!(config.get_shortcut("Test").is_some());
        assert!(config.get_shortcut("test").is_none());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = AppConfig::default();
        config.set_shortcut("home", "/home/user").unwrap();
        config.default_file_manager = Some("dolphin".to_string());
        
        // Serialize to TOML
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("home"));
        assert!(toml_str.contains("dolphin"));
        
        // Deserialize
        let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
        assert!(parsed.shortcuts.contains_key("home"));
        assert_eq!(parsed.default_file_manager, Some("dolphin".to_string()));
    }

    #[test]
    fn test_json_serialization() {
        let mut config = AppConfig::default();
        config.set_shortcut("work", "/home/user/work").unwrap();
        
        let json = config.to_json().unwrap();
        assert!(json.contains("work"));
        
        let parsed = AppConfig::from_json(&json).unwrap();
        assert!(parsed.shortcuts.contains_key("work"));
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = AppConfig::default();
        config1.set_shortcut("a", "/path/a").unwrap();
        
        let mut config2 = AppConfig::default();
        config2.set_shortcut("b", "/path/b").unwrap();
        config2.default_file_manager = Some("nautilus".to_string());
        
        config1.merge(config2);
        
        assert!(config1.shortcuts.contains_key("a"));
        assert!(config1.shortcuts.contains_key("b"));
        assert_eq!(config1.default_file_manager, Some("nautilus".to_string()));
    }

    #[test]
    fn test_set_and_get_value() {
        let mut config = AppConfig::default();
        
        config.set_value("shell.enabled", "false").unwrap();
        assert_eq!(config.get_value("shell.enabled").unwrap(), "false");
        
        config.set_value("behavior.confirm_overwrite", "false").unwrap();
        assert_eq!(config.get_value("behavior.confirm_overwrite").unwrap(), "false");
    }

    #[test]
    fn test_invalid_config_key() {
        let mut config = AppConfig::default();
        
        assert!(config.set_value("invalid.key", "value").is_err());
        assert!(config.get_value("invalid.key").is_err());
    }

    #[test]
    fn test_file_manager_detection() {
        let config = AppConfig::default();
        let fm = config.get_file_manager();
        
        // Should return a non-empty string
        assert!(!fm.is_empty());
        
        // Should be one of the known file managers
        let known = vec![
            "explorer", "open", "xdg-open", "nautilus", "dolphin",
            "thunar", "pcmanfm", "nemo", "caja"
        ];
        assert!(known.contains(&fm.as_str()));
    }
}
