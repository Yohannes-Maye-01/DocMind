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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Index { path } => commands::index(path)?,
        Commands::Search { query } => commands::search(query)?,
        Commands::Stats => commands::stats()?,
    }

    Ok(())
}