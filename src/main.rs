mod cli;
mod commands;
mod error;
mod scanner;
mod storage;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let current_dir = std::env::current_dir()?;

    match cli.command {
        Commands::Push { message, dry_run } => {
            commands::push::run(&current_dir, message.as_deref(), dry_run).await?;
        }
        Commands::Log { project, limit } => {
            commands::log::run(project.as_deref(), limit).await?;
        }
        Commands::Checkout { snapshot, output } => {
            commands::checkout::run(&snapshot, output.as_deref()).await?;
        }
        Commands::Init => {
            commands::init::run(&current_dir)?;
        }
        Commands::Status => {
            commands::status::run(&current_dir).await?;
        }
        Commands::Clone { project } => {
            commands::clone::run(&project, &current_dir).await?;
        }
    }

    Ok(())
}
