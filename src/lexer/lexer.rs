//! Lexer
//!
//! This module is responsible for converting a given program into a sequnce
//! of tokens.

pub struct Lexer<'a> {
    pub source: &'a str,
    pub position: usize,
}
