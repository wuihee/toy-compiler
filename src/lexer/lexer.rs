//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.

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

        match symbol {
            'c' => self.match_class(),
            _ => todo!(),
        }
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn match_class(&mut self) -> Result<Token, LexerError> {
        let keyword = String::from("class");

        for expected_symbol in keyword.chars() {
            let Some(symbol) = self.peek() else {
                return Err(LexerError::UnexpectedToken);
            };

            if symbol == expected_symbol {
                self.advance();
                continue;
            }

            return Err(LexerError::UnexpectedToken);
        }

        Ok(Token {
            kind: TokenKind::Keyworkd(Keyword::Class),
            span: Span { start: 0, end: 0 },
        })
    }
}
