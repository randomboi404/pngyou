//! PNG You! CLI
//!
//! This binary provides a command-line interface for hiding and extracting
//! data inside PNG files using the `pngyou` library.

mod args;
mod commands;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode {
            input,
            output,
            chunk_type,
            message,
        } => commands::encode(input, output, chunk_type, message),
        Commands::Decode { input, chunk_type } => commands::decode(input, chunk_type),
        Commands::Remove {
            input,
            output,
            chunk_type,
        } => commands::remove(input, output, chunk_type),
        Commands::Print { input } => commands::print(input),
    }
}
