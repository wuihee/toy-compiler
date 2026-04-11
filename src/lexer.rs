//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.
//!
//! ## Invariants
//!
//! - **Longest Valid Token**: The lexer always matches the longest valid token.
//! - **Position**: `position` always points to the next unread symbol.
//!
//! ## State
//!
//! - `source`: The source program to be scanned.
//! - `position`: A cursor into the source program string.
//!
//! ## Transitions
//!
//! - **Scan**: Lookahead starting from `position`.
//! - **Consume**: Advance `position` and return the token consumed.

pub mod keywords;
pub mod token;

use crate::lexer::{
    keywords::lookup_keyword,
    token::{Span, Token, TokenKind},
};

/// This struct converts a program into a stream of [`Token`]s.
pub struct Lexer<'a> {
    /// The source program as string.
    source: &'a str,

    /// A cursor into `source`.
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Instantiate a new `Lexer`.
    ///
    /// # Arguments
    ///
    /// - `source`: The source program to scan.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use toy_compiler::lexer::Lexer;
    ///
    /// let source = "int x = 0;";
    /// let lexer = Lexer::new(source);
    /// ```
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            position: 0,
        }
    }

    /// Pull the next token from the `source` program.
    ///
    /// # Examples (TODO)
    ///
    /// ```rust
    /// ```
    pub fn next_token(&mut self) -> Token {
        // Remove whitespace.
        while matches!(self.peek(), Some(symbol) if symbol.is_whitespace()) {
            self.advance();
        }

        // No tokens remaining.
        let Some(symbol) = self.peek() else {
            return Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: self.position,
                    end: self.position,
                },
            };
        };

        match symbol {
            '0'..='9' => self.consume_integer(),
            'a'..='z' | 'A'..='Z' => self.consume_identifier(),
            '+' => self.consume(TokenKind::Plus),
            '-' => self.consume(TokenKind::Minus),
            '*' => self.consume(TokenKind::Star),
            '=' => self.consume(TokenKind::Equal),
            '<' => self.consume(TokenKind::LessThan),
            '!' => self.consume(TokenKind::Bang),
            '(' => self.consume(TokenKind::LeftParenthesis),
            ')' => self.consume(TokenKind::RightParenthesis),
            '[' => self.consume(TokenKind::LeftBracket),
            ']' => self.consume(TokenKind::RightBracket),
            '{' => self.consume(TokenKind::LeftBrace),
            '}' => self.consume(TokenKind::RightBrace),
            ',' => self.consume(TokenKind::Comma),
            '.' => self.consume(TokenKind::Dot),
            ';' => self.consume(TokenKind::Semicolon),
            '&' if matches!(self.peek_by(1), Some('&')) => self.consume(TokenKind::And),
            _ => self.consume(TokenKind::Unknown(symbol)),
        }
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.peek_by(0)
    }

    /// Get the symbol at `position + n`.
    fn peek_by(&self, n: usize) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position + n)
            .map(|&symbol| symbol as char)
    }

    /// Advance the cursor by one position.
    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_by(&mut self, n: usize) {
        self.position += n;
    }

    /// Consume the next keyword or identifier.
    fn consume_identifier(&mut self) -> Token {
        let start = self.position;

        while matches!(self.peek(), Some(symbol) if symbol.is_alphanumeric()) {
            self.advance();
        }

        let identifier = &self.source[start..self.position];
        let kind =
            lookup_keyword(&identifier).unwrap_or(TokenKind::Identifier(identifier.to_string()));

        Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        }
    }

    /// Consume the next integer literal.
    fn consume_integer(&mut self) -> Token {
        let start = self.position;

        while matches!(self.peek(), Some(symbol) if symbol.is_ascii_digit()) {
            self.advance();
        }

        let &integer = &self.source[start..self.position]
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

    /// TODO
    /// We are still checking token length and this is still expectation-driven?
    /// But how else am I supposed to know the length of tokens?
    fn consume(&mut self, kind: TokenKind) -> Token {
        let length = kind.lexeme().map_or(1, str::len);
        let start = self.position;
        let end = start + length;

        self.advance_by(length);

        Token {
            kind,
            span: Span { start, end },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eof() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token();

        assert_eq!(
            token,
            Token {
                kind: TokenKind::Eof,
                span: Span { start: 0, end: 0 }
            }
        )
    }
}
