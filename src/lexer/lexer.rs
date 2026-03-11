//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.
//!
//! ## Goal
//!
//! Convert a source program string into a stream of `Token`s.
//!
//! ## Invariants
//!
//! - **Longest Valid Token**: The lexer always matches the longest valid
//! token.
//! - **Loop**: `position` points to the start of the next token.
//!
//! ## State
//!
//! - `source`: The source program to be scanned.
//! - `position`: A cursor into the source program string.
//!
//! ## Transitions
//!
//! - For all symbols `s` in `source`,
//! - Match `s`
//!   - letter => scan_identifier(), note that `scan_identifier` extracts a
//! "character class" and not a specific keyword.

use crate::lexer::{
    errors::LexerError,
    token::{Keyword, Span, Token, TokenKind},
};

/// This struct converts a program into a stream of `Token`s.
pub struct Lexer<'a> {
    /// The source program as string.
    source: &'a str,

    /// A cursor into `source`.
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Instantiate a new `Lexer`.
    ///
    /// # Examples
    ///
    /// ```rs
    /// use toy_compiler::Lexer;
    ///
    /// let source = "int x = 0;"
    /// let lexer = Lexer::new(source);
    /// ```
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            position: 0,
        }
    }

    /// Get the next token from the `source` program.
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        let Some(symbol) = self.peek() else {
            return Err(LexerError::UnexpectedToken);
        };

        if symbol.is_ascii_alphabetic() {
            self.scan_identifier();
        }

        todo!()
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    fn scan_identifier(&mut self) {
        match self.peek() {
            Some('b') => self.scan_boolean(),
            _ => todo!(),
        }
    }

    fn scan_boolean(&mut self) {}
}
