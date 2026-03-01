//! Lexer
//!
//! This module is responsible for converting a string into a sequnce of tokens.

use crate::lexer::token::Token;

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(tokens: Vec<Token>) -> Lexer {
        Lexer { tokens }
    }
}
