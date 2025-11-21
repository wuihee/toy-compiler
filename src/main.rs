use std::process;

use clap::Parser;

use toy_compiler::{
    cli::{Cli, Command},
    scan_file,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Scan { file }) => {
            if let Err(error) = scan_file(&file) {
                eprintln!("Scan error: {error}");
                process::exit(1);
            }
        }
        None => {}
    }
}
