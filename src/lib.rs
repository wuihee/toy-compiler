use std::{error::Error, fs, path::Path};

use crate::lexer::{Lexer, token::Token};

pub mod ast;
pub mod cli;
pub mod lexer;
pub mod parser;
pub mod span;

/// Scans a MiniJava file and prints out the tokens.
pub fn scan_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let tokens: Vec<Token> = Lexer::new(&source).collect();

    for token in tokens {
        println!("{token:?} ");
    }

    Ok(())
}
