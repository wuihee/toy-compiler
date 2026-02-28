use clap::Parser;

use toy_compiler::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Scan { file }) => {}
        _ => {}
    }
}
