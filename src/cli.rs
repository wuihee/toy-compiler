//! # CLI
//!
//! Defines CLI commands for the program.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Represents the top-level CLI entry point.
#[derive(Parser)]
#[command(about = "Demonstrates features of the toy compiler")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Represents the different commands for the toy compiler.
#[derive(Subcommand)]
pub enum Command {
    /// Scans a given file and ouputs the tokens.
    Scan {
        #[arg(value_name = "file", help = "Input source file")]
        file: PathBuf,
    },
}
