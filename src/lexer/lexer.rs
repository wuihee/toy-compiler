//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.
//!
//! ## Invariants
//!
//! - **Position**: `position` always points to the next unread symbol
//! - **Longest Valid Token**: The lexer always matches the longest valid
//! token.
//! - **Loop**: `position` points to the start of the next token after each
//! call to `next_token`.
//!
//! ## State
//!
//! - `source`: The source program to be scanned.
//! - `position`: A cursor into the source program string.

use crate::lexer::{
    errors::LexerError,
    keywords::lookup_keyword,
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
            'a'..'z' | 'A'..'Z' => Ok(self.next_identifier()),

            // `symbol` is unrecognized in the language.
            symbol @ _ => {
                let position = self.position;
                Err(LexerError::UnexpectedSymbol {
                    symbol,
                    span: Span {
                        start: position,
                        end: position,
                    },
                })
            }
        }
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    /// Consume the next keyword or identifier.
    fn next_identifier(&mut self) -> Token {
        // Check if the next token is `System.out.println`.
        if let Some(token) = self.scan_system_out_println() {
            return token;
        }

        let start = self.position;
        let mut identifier = String::new();

        while let Some(symbol) = self.peek() {
            if !symbol.is_alphanumeric() {
                break;
            }

            identifier.push(symbol);
            self.position += 1;
        }

        let kind = lookup_keyword(&identifier).unwrap_or(TokenKind::Identifier(identifier));

        Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        }
    }

    /// Consume the next token if it's `System.out.println`.
    ///
    /// This helper function is needed because of the decision to treat
    /// `System.out.println` as a single keyword.
    fn scan_system_out_println(&mut self) -> Option<Token> {
        let keyword = "System.out.println";
        let length = keyword.len();
        let start = self.position;
        let end = self.position + length;

        if let Some(slice) = self.source.get(start..end) {
            if slice == keyword {
                self.position += length;
                return Some(Token {
                    kind: TokenKind::Keyword(Keyword::SystemOutPrintln),
                    span: Span { start, end },
                });
            }
        }

        None
    }
}
