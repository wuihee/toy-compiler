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
    keywords,
    token::{Span, Token, TokenKind},
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
        // End of file if there are no symbols left to read.
        let Some(symbol) = self.peek() else {
            let position = self.position;
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: position,
                    end: position,
                },
            });
        };

        // Try to return the next token.
        match symbol {
            'S' => {
                let identifier = self.scan_system_out_println();

                todo!()
            }

            'a'..'z' | 'A'..'Z' => {
                let identifier = self.scan_identifier();
                let kind = keywords::lookup_keyword(&identifier)
                    .unwrap_or(TokenKind::Identifier(identifier.clone()));

                todo!("Why does this feel wrong?");
                let size = identifier.len();
                let start = self.position;
                let end = start + size;

                self.position += size;

                Ok(Token {
                    kind,
                    span: Span { start, end },
                })
            }

            _ => Err(LexerError::UnexpectedToken),
        }
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    /// Scan the next identifier.
    fn scan_identifier(&self) -> String {
        let source_bytes = self.source.as_bytes();
        let mut i = self.position;
        let mut identifier = String::new();

        while let Some(symbol) = source_bytes.get(i).map(|&symbol| symbol as char) {
            if !symbol.is_alphanumeric() {
                break;
            }

            identifier.push(symbol);
            i += 1;
        }

        identifier
    }

    /// Check if the next token is `System.out.println`.
    fn scan_system_out_println(&self) {}
}
