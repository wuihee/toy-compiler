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
    /// # Examples
    ///
    /// TODO
    ///
    /// ```rust
    /// ```
    pub fn next_token(&mut self) -> Token {
        // Remove whitespace.
        while matches!(self.peek(), Some(symbol) if symbol.is_whitespace()) {
            self.bump();
        }

        let start = self.position;

        // No tokens remaining.
        let Some(symbol) = self.bump() else {
            return Token {
                kind: TokenKind::Eof,
                span: Span {
                    start,
                    end: self.position,
                },
            };
        };

        match symbol {
            '0'..='9' => self.consume_integer(start),
            'a'..='z' | 'A'..='Z' => self.consume_identifier(start),
            '+' => Token::new(TokenKind::Plus, Span::new(start, self.position)),
            '-' => Token::new(TokenKind::Minus, Span::new(start, self.position)),
            '*' => Token::new(TokenKind::Star, Span::new(start, self.position)),
            '=' => Token::new(TokenKind::Equal, Span::new(start, self.position)),
            '<' => Token::new(TokenKind::LessThan, Span::new(start, self.position)),
            '!' => Token::new(TokenKind::Bang, Span::new(start, self.position)),
            '(' => Token::new(TokenKind::LeftParenthesis, Span::new(start, self.position)),
            ')' => Token::new(TokenKind::RightParenthesis, Span::new(start, self.position)),
            '[' => Token::new(TokenKind::LeftBracket, Span::new(start, self.position)),
            ']' => Token::new(TokenKind::RightBracket, Span::new(start, self.position)),
            '{' => Token::new(TokenKind::LeftBrace, Span::new(start, self.position)),
            '}' => Token::new(TokenKind::RightBrace, Span::new(start, self.position)),
            ',' => Token::new(TokenKind::Comma, Span::new(start, self.position)),
            '.' => Token::new(TokenKind::Dot, Span::new(start, self.position)),
            ';' => Token::new(TokenKind::Semicolon, Span::new(start, self.position)),
            '&' if matches!(self.bump(), Some('&')) => {
                Token::new(TokenKind::And, Span::new(start, self.position))
            }
            _ => Token::new(TokenKind::Unknown(symbol), Span::new(start, self.position)),
        }
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    /// Return the symbol at the current `position` and advance.
    fn bump(&mut self) -> Option<char> {
        if let Some(symbol) = self.peek() {
            self.position += 1;
            return Some(symbol);
        }

        None
    }

    /// Consume the next keyword or identifier.
    fn consume_identifier(&mut self, start: usize) -> Token {
        while matches!(self.peek(), Some(symbol) if symbol.is_alphanumeric() || symbol == '_') {
            self.bump();
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
    fn consume_integer(&mut self, start: usize) -> Token {
        while matches!(self.peek(), Some(symbol) if symbol.is_ascii_digit()) {
            self.bump();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexeme(source: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();

        assert_eq!(token, Token::new(kind, Span::new(0, source.len())))
    }

    #[test]
    fn eof() {
        test_lexeme("", TokenKind::Eof);
    }

    #[test]
    fn integer() {
        test_lexeme("42", TokenKind::IntegerLiteral(42));
        test_lexeme("007", TokenKind::IntegerLiteral(7));
    }

    #[test]
    fn identifier() {
        test_lexeme("hello", TokenKind::Identifier(String::from("hello")));
        test_lexeme("hell0", TokenKind::Identifier(String::from("hell0")));
        test_lexeme("hell_o", TokenKind::Identifier(String::from("hell_o")));
    }

    #[test]
    fn keywords() {
        test_lexeme("boolean", TokenKind::Keyword(token::KeywordKind::Boolean));
        test_lexeme("class", TokenKind::Keyword(token::KeywordKind::Class));
        test_lexeme("else", TokenKind::Keyword(token::KeywordKind::Else));
        test_lexeme("extends", TokenKind::Keyword(token::KeywordKind::Extends));
        test_lexeme("if", TokenKind::Keyword(token::KeywordKind::If));
        test_lexeme("int", TokenKind::Keyword(token::KeywordKind::Int));
        test_lexeme("main", TokenKind::Keyword(token::KeywordKind::Main));
        test_lexeme("new", TokenKind::Keyword(token::KeywordKind::New));
        test_lexeme("public", TokenKind::Keyword(token::KeywordKind::Public));
        test_lexeme("return", TokenKind::Keyword(token::KeywordKind::Return));
        test_lexeme("static", TokenKind::Keyword(token::KeywordKind::Static));
        test_lexeme("this", TokenKind::Keyword(token::KeywordKind::This));
        test_lexeme("void", TokenKind::Keyword(token::KeywordKind::Void));
        test_lexeme("while", TokenKind::Keyword(token::KeywordKind::While));
    }

    #[test]
    fn operators_and_delimiters() {
        test_lexeme("+", TokenKind::Plus);
        test_lexeme("-", TokenKind::Minus);
        test_lexeme("*", TokenKind::Star);
        test_lexeme("=", TokenKind::Equal);
        test_lexeme("&&", TokenKind::And);
        test_lexeme("<", TokenKind::LessThan);
        test_lexeme("!", TokenKind::Bang);
        test_lexeme("(", TokenKind::LeftParenthesis);
        test_lexeme(")", TokenKind::RightParenthesis);
        test_lexeme("[", TokenKind::LeftBracket);
        test_lexeme("]", TokenKind::RightBracket);
        test_lexeme("{", TokenKind::LeftBrace);
        test_lexeme("}", TokenKind::RightBrace);
        test_lexeme(",", TokenKind::Comma);
        test_lexeme(".", TokenKind::Dot);
        test_lexeme(";", TokenKind::Semicolon);
    }

    #[test]
    fn unknown() {
        test_lexeme("%", TokenKind::Unknown('%'));
    }
}
