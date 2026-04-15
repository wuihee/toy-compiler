use clap::Parser;

use toy_compiler::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    if let Some(Command::Scan { file }) = cli.command {
        toy_compiler::scan_file(&file).expect("Failed to read file");
    }
}
