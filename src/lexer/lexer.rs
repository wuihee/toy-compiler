//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.

use crate::lexer::{
    errors::LexerError,
    token::{Span, Token, TokenKind},
};

pub struct Lexer<'a> {
    pub source: &'a str,
    pub position: usize,
}

// Correct lifetimes?
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        Ok(Token {
            kind: TokenKind::Eof,
            span: Span { start: 0, end: 1 },
        })
    }
}
