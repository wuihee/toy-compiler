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
//!
//! ## State
//!
//! - `source`: The source program to be scanned.
//! - `position`: A cursor into the source program string.

use crate::lexer::{
    errors::LexerError,
    keywords::lookup_keyword,
    token::{Delimiter, Keyword, Operator, Span, Token, TokenKind},
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

        // TODO: Deal with comments!
        match symbol {
            'a'..='z' | 'A'..='Z' => Ok(self.next_identifier()),
            '0'..'9' => Ok(self.next_integer()),
            '+' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::Add))),
            '-' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::Subtract))),
            '*' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::Multiply))),
            '=' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::Assign))),
            '!' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::Not))),
            '<' => Ok(self.make_one_char_token(TokenKind::Operator(Operator::LessThan))),
            // TODO: &&
            '(' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftParenthesis))),
            ')' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightParenthesis))),
            '[' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftBracket))),
            ']' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightBracket))),
            '{' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftBrace))),
            '}' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightBrace))),
            ',' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::Comma))),
            '.' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::Dot))),
            ';' => Ok(self.make_one_char_token(TokenKind::Delimiter(Delimiter::Semicolon))),

            // The current `symbol` is unrecognized in the language.
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

    /// Consume the next integer literal.
    fn next_integer(&mut self) -> Token {
        let start = self.position;
        let mut integer = String::new();

        while let Some(symbol) = self.peek() {
            if !symbol.is_ascii_digit() {
                break;
            }

            integer.push(symbol);
            self.position += 1;
        }

        let integer = integer
            .parse::<i64>()
            .expect("This error should not be possible");
        let kind = TokenKind::IntegerLiteral(integer);

        Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        }
    }

    /// Consume the next single character token.
    fn make_one_char_token(&mut self, kind: TokenKind) -> Token {
        let start = self.position;
        self.position += 1;

        Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        }
    }
}
