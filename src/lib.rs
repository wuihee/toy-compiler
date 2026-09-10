use std::{error::Error, fs, path::Path};

use crate::{
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::Parser,
};

pub mod ast;
pub mod cli;
pub mod lexer;
pub mod parser;
pub mod span;

/// Scans a MiniJava file and prints out the tokens.
pub fn scan_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let mut tokens = Vec::<Token>::new();
    let mut lexer = Lexer::new(&source);

    while let token = lexer.next_token()
        && token.kind != TokenKind::Eof
    {
        tokens.push(token);
    }

    for token in tokens {
        println!("{token:?} ");
    }

    Ok(())
}

/// Reads and parses a MiniJava file and prints the AST.
pub fn parse_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let program = Parser::new(Lexer::new(&source)).parse()?;

    println!("{program:#?}");

    Ok(())
}
