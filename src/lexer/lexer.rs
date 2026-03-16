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
    ///
    /// # Examples
    ///
    /// TODO
    ///
    /// ```rs
    /// ```
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
            'a'..='z' | 'A'..='Z' => self.next_identifier(),
            '0'..'9' => self.next_integer(),
            '+' => self.make_one_char_token(TokenKind::Operator(Operator::Add)),
            '-' => self.make_one_char_token(TokenKind::Operator(Operator::Subtract)),
            '*' => self.make_one_char_token(TokenKind::Operator(Operator::Multiply)),
            '=' => self.make_one_char_token(TokenKind::Operator(Operator::Assign)),
            '!' => self.make_one_char_token(TokenKind::Operator(Operator::Not)),
            '<' => self.make_one_char_token(TokenKind::Operator(Operator::LessThan)),
            '&' => self.scan("&&", TokenKind::Operator(Operator::Add)).ok_or(
                LexerError::UnexpectedSymbol {
                    symbol,
                    span: Span {
                        start: self.position,
                        end: self.position + 1,
                    },
                },
            ),
            '(' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftParenthesis)),
            ')' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightParenthesis)),
            '[' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftBracket)),
            ']' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightBracket)),
            '{' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::LeftBrace)),
            '}' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::RightBrace)),
            ',' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::Comma)),
            '.' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::Dot)),
            ';' => self.make_one_char_token(TokenKind::Delimiter(Delimiter::Semicolon)),

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
    fn next_identifier(&mut self) -> Result<Token, LexerError> {
        // Check if the next token is `System.out.println`.
        if let Some(token) = self.scan(
            "System.out.println",
            TokenKind::Keyword(Keyword::SystemOutPrintln),
        ) {
            return Ok(token);
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

        Ok(Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        })
    }

    /// Consume the next integer literal.
    fn next_integer(&mut self) -> Result<Token, LexerError> {
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

        Ok(Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        })
    }

    /// Consume the next `lexeme` as token `kind` if available.
    fn scan(&mut self, lexeme: &str, kind: TokenKind) -> Option<Token> {
        let length = lexeme.len();
        let start = self.position;
        let end = start + length;

        if let Some(slice) = self.source.get(start..end) {
            if slice == lexeme {
                self.position += length;
                return Some(Token {
                    kind,
                    span: Span { start, end },
                });
            }
        }

        None
    }

    /// Consume the next single character token.
    fn make_one_char_token(&mut self, kind: TokenKind) -> Result<Token, LexerError> {
        let start = self.position;
        self.position += 1;

        Ok(Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        })
    }
}
