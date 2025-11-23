use std::{error::Error, fs, path::Path};

use crate::lexer::lexer::Lexer;

pub mod ast;
pub mod cli;
pub mod lexer;
pub mod parser;

/// Scans an input file and prints out the tokens.
pub fn scan_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let tokens = Lexer::new(&input).scan()?;

    for token in tokens {
        print!("{token:?} ");
    }

    Ok(())
}
