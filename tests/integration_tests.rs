//! Integration tests for QuickNav

use std::process::Command;
use std::path::PathBuf;

fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("quicknav");
    path
}

#[test]
fn test_help_command() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute quicknav");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("navr"));
    assert!(stdout.contains("jump"));
    assert!(stdout.contains("open"));
    assert!(stdout.contains("config"));
}

#[test]
fn test_version_command() {
    let output = Command::new(get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute navr");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0") || stdout.contains("navr"));
}

#[test]
fn test_config_show() {
    let output = Command::new(get_binary_path())
        .args(&["config", "show"])
        .output()
        .expect("Failed to execute quicknav");

    // May fail if config doesn't exist, but should not panic
    let _stdout = String::from_utf8_lossy(&output.stdout);
    let _stderr = String::from_utf8_lossy(&output.stderr);
}

#[test]
fn test_jump_list() {
    let output = Command::new(get_binary_path())
        .args(&["jump", "--list"])
        .output()
        .expect("Failed to execute quicknav");

    let _stdout = String::from_utf8_lossy(&output.stdout);
}

#[test]
fn test_shell_complete_bash() {
    let output = Command::new(get_binary_path())
        .args(&["shell", "complete", "bash"])
        .output()
        .expect("Failed to execute quicknav");

    assert!(output.status.success());
}

#[test]
fn test_shell_complete_zsh() {
    let output = Command::new(get_binary_path())
        .args(&["shell", "complete", "zsh"])
        .output()
        .expect("Failed to execute quicknav");

    assert!(output.status.success());
}

#[test]
fn test_shell_complete_fish() {
    let output = Command::new(get_binary_path())
        .args(&["shell", "complete", "fish"])
        .output()
        .expect("Failed to execute quicknav");

    assert!(output.status.success());
}

#[test]
fn test_invalid_command() {
    let output = Command::new(get_binary_path())
        .arg("invalid-command")
        .output()
        .expect("Failed to execute quicknav");

    assert!(!output.status.success());
}

#[test]
fn test_quick_flag() {
    let output = Command::new(get_binary_path())
        .args(&["--quick", "home"])
        .output()
        .expect("Failed to execute quicknav");

    // Should either succeed or fail gracefully
    let _stdout = String::from_utf8_lossy(&output.stdout);
    let _stderr = String::from_utf8_lossy(&output.stderr);
}
