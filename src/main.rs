use std::process;

use clap::Parser;

use toy_compiler::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Scan { file }) => {
            if let Err(error) = toy_compiler::scan_file(&file) {
                eprintln!("{error}");
                process::exit(1);
            }
        }
        Some(Command::Parse { file }) => {
            if let Err(error) = toy_compiler::parse_file(&file) {
                eprintln!("{error}");
                process::exit(1);
            }
        }
        None => {}
    }
}
