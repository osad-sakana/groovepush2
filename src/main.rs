mod cli;
mod commands;
mod error;
mod scanner;
mod storage;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let current_dir = std::env::current_dir()?;

    match cli.command {
        Commands::Commit { message, dry_run } => {
            commands::commit::run(&current_dir, message.as_deref(), dry_run)?;
        }
        Commands::Log { limit } => {
            commands::log::run(limit)?;
        }
        Commands::Checkout { snapshot } => {
            commands::checkout::run(&snapshot)?;
        }
        Commands::Init => {
            commands::init::run(&current_dir)?;
        }
        Commands::Status => {
            commands::status::run(&current_dir)?;
        }
    }

    Ok(())
}
