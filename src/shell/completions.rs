//! Dynamic completion generation for shortcuts

// use anyhow::Result;
// use std::collections::HashMap;

// use crate::config::AppConfig;

// /// Generate completion candidates for a partial shortcut name
// pub fn complete_shortcut(partial: &str, config: &AppConfig) -> Vec<String> {
//     let partial_lower = partial.to_lowercase();
    
//     config
//         .shortcuts
//         .keys()
//         .filter(|name| {
//             let name_lower = name.to_lowercase();
//             name_lower.starts_with(&partial_lower) || 
//             name_lower.contains(&partial_lower)
//         })
//         .cloned()
//         .collect()
// }

// /// Generate completion for all shortcuts
// pub fn complete_all_shortcuts(config: &AppConfig) -> Vec<(String, String)> {
//     config
//         .shortcuts
//         .iter()
//         .map(|(name, path)| (name.clone(), path.clone()))
//         .collect()
// }

// /// Generate completion script for bash dynamic completion
// pub fn generate_bash_dynamic_completion() -> String {
//     r#"
// # Navr dynamic completion for shortcuts
// _navr_complete() {
//     local cur="${COMP_WORDS[COMP_CWORD]}"
//     local shortcuts=$(navr jump --list 2>/dev/null | grep -E '^\s+\w+' | awk '{print $1}')
//     COMPREPLY=($(compgen -W "$shortcuts" -- "$cur"))
// }

// complete -F _navr_complete navr
// complete -F _navr_complete j
// "#.to_string()
// }

// /// Generate completion script for zsh dynamic completion
// pub fn generate_zsh_dynamic_completion() -> String {
//     r#"
// # Navr dynamic completion for shortcuts
// _navr_complete() {
//     local -a shortcuts
//     shortcuts=(${(f)"$(navr jump --list 2>/dev/null | grep -E '^\s+\w+' | awk '{print $1}')"})
//     _describe 'shortcut' shortcuts
// }

// compdef _navr_complete navr
// compdef _navr_complete j
// "#.to_string()
// }

// /// Generate completion script for fish dynamic completion
// pub fn generate_fish_dynamic_completion() -> String {
//     r#"
// # Navr dynamic completion for shortcuts
// complete -c navr -n '__fish_use_subcommand' -a 'jump open config shell export import'
// complete -c navr -n '__fish_seen_subcommand_from jump' -a '(navr jump --list 2>/dev/null | string match -r "^\s+(\w+)" | string replace -r "^\s+" "")'
// complete -c j -a '(navr jump --list 2>/dev/null | string match -r "^\s+(\w+)" | string replace -r "^\s+" "")'
// "#.to_string()
// }

// /// Get completion description for a shortcut
// pub fn get_shortcut_description(name: &str, config: &AppConfig) -> Option<String> {
//     config.shortcuts.get(name).map(|path| {
//         // Truncate long paths
//         if path.len() > 50 {
//             format!("...{}", &path[path.len() - 47..])
//         } else {
//             path.clone()
//         }
//     })
// }

// /// Generate a completion cache file
// pub fn generate_completion_cache(config: &AppConfig) -> Result<String> {
//     let mut cache = String::new();
    
//     for (name, path) in &config.shortcuts {
//         cache.push_str(&format!("{}\t{}\n", name, path));
//     }
    
//     Ok(cache)
// }

// /// Parse completion cache
// pub fn parse_completion_cache(cache: &str) -> HashMap<String, String> {
//     let mut shortcuts = HashMap::new();
    
//     for line in cache.lines() {
//         let parts: Vec<&str> = line.split('\t').collect();
//         if parts.len() >= 2 {
//             shortcuts.insert(parts[0].to_string(), parts[1].to_string());
//         }
//     }
    
//     shortcuts
// }
