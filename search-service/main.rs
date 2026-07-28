//! Search service CLI application for DocMind.
//!
//! This module provides a command-line interface for indexing documents
//! and performing searches against the document index.

use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod index;
mod search;

use cli::{Cli, Commands};

fn main() {
    // Initialize tracing or logging subscriber here if needed, e.g.:
    // tracing_subscriber::fmt::init();

    if let Err(err) = try_main() {
        // Print the error chain cleanly in a professional format
        epic!("Error: {:#?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Index { path } => {
            commands::index(&path)?;
        }
        Commands::Search { query } => {
            commands::search(&query)?;
        }
        Commands::Stats => {
            commands::stats()?;
        }
    }

    Ok(())
}
