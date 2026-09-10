use clap::Parser;

use toy_compiler::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Scan { file }) => {
            toy_compiler::scan_file(&file).expect("Failed to read file");
        }

        Some(Command::Parse { file }) => {
            toy_compiler::parse_file(&file).expect("Failed to parse file")
        }

        _ => println!("Unexpected command"),
    }
}
