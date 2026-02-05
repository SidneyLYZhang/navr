//! Navr - A fast directory navigation tool
//!
//! This tool provides quick directory jumping, file manager integration,
//! and cross-platform shell support.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use owo_colors::OwoColorize;
use std::process;

mod commands;
mod config;
mod platform;
mod shell;

use commands::{config::{ConfigCommand, ConfigSubCommand, ShellSubCommand}, jump::JumpCommand, open::OpenCommand};
use config::AppConfig;

/// Navr - Fast directory navigation tool
#[derive(Parser, Debug)]
#[command(
    name = "navr",
    about = "A fast directory navigation tool with cross-platform support",
    long_about = "Navr allows you to quickly jump between directories, \
                  open file managers, and manage navigation shortcuts.",
    version,
    author,
    help_template = "{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
{usage}

{all-args}{after-help}",
    arg_required_else_help = false
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true, help = "Enable verbose logging")]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true, help = "Path to custom config file")]
    config: Option<String>,

    /// Quick open mode - directly open directory or shortcut
    #[arg(
        short = 'k',
        long = "quick",
        help = "Quickly open a directory or shortcut"
    )]
    quick: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Jump to a directory or shortcut
    #[command(visible_alias = "j")]
    Jump {
        /// Target directory or shortcut name
        target: Option<String>,

        /// List all available shortcuts
        #[arg(short, long)]
        list: bool,

        /// Add current directory as shortcut
        #[arg(short, long, value_name = "NAME")]
        add: Option<String>,

        /// Remove a shortcut
        #[arg(short, long, value_name = "NAME")]
        remove: Option<String>,
    },

    /// Open directory in file manager
    #[command(visible_alias = "o")]
    Open {
        /// Directory or shortcut to open
        target: Option<String>,

        /// Open with specific file manager
        #[arg(short, long)]
        with: Option<String>,
},

    /// Configuration management
    #[command(visible_alias = "cfg")]
    Config {
        #[command(subcommand)]
        action: ConfigSubCommand,
    },

    /// Shell integration commands
    #[command(visible_alias = "sh")]
    Shell {
        #[command(subcommand)]
        action: ShellSubCommand,
    },

    /// Import/Export configuration
    #[command(visible_alias = "exp")]
    Export {
        /// Export format (json, toml, yaml)
        #[arg(short, long, default_value = "toml")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Import configuration
    #[command(visible_alias = "imp")]
    Import {
        /// Input file path
        input: String,

        /// Merge with existing config
        #[arg(short, long)]
        merge: bool,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Load configuration
    let mut config = if let Some(config_path) = &cli.config {
        AppConfig::load_from_path(config_path)?
    } else {
        AppConfig::load()?
    };

    // Handle quick mode (-k/--quick)
    if let Some(quick_target) = cli.quick {
        let cmd = OpenCommand::new(quick_target);
        return cmd.execute(&config);
    }

    // Execute subcommand or show help
    match cli.command {
        Some(Commands::Jump {
            target,
            list,
            add,
            remove,
        }) => {
            let cmd = JumpCommand::new(target, list, add, remove);
            cmd.execute(&mut config)?;
        }
        Some(Commands::Open { target, with }) => {
            let target = target.unwrap_or_else(|| ".".to_string());
            let cmd = OpenCommand::with_manager(target, with);
            cmd.execute(&config)?;
        }
        Some(Commands::Config { action }) => {
            let cmd = ConfigCommand::new(action);
            cmd.execute(&mut config)?;
        }
        Some(Commands::Shell { action }) => {
            handle_shell_command(action)?;
        }
        Some(Commands::Export { format, output }) => {
            commands::export::execute(&config, &format, output.as_deref())?;
        }
        Some(Commands::Import { input, merge }) => {
            commands::import::execute(&mut config, &input, merge)?;
        }
        None => {
            // No subcommand - interactive mode or show help
            Cli::command().print_help()?;
            println!();
        }
    }

    Ok(())
}

fn handle_shell_command(action: ShellSubCommand) -> Result<()> {
    match action {
        ShellSubCommand::Complete { shell } => {
            shell::generate_completions(shell)?;
        }
        ShellSubCommand::Install { shell, path } => {
            shell::install_integration(shell, path.as_deref())?;
        }
        ShellSubCommand::Init { shell } => {
            shell::print_init_script(shell)?;
        }
    }
    Ok(())
}

fn init_logging(verbose: bool) {
    let filter = if verbose {
        "debug"
    } else {
        "info"
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
