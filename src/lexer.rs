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

pub mod errors;
pub mod keywords;
pub mod token;

use crate::lexer::{
    errors::LexerError,
    keywords::lookup_keyword,
    token::{DelimeterKind, KeywordKind, OperatorKind, Span, Token, TokenKind},
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

    /// Get the next token from the `source` program.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use toy_compiler::lexer::{
    ///     Lexer,
    ///     token::{Keyword, Span, Token, TokenKind},
    /// };
    ///
    /// let source = "int x = 0;";
    /// let mut lexer = Lexer::new(source);
    ///
    /// // Get next token.
    /// let token = lexer.next_token();
    /// assert_eq!(
    ///     token,
    ///     Ok(Token {
    ///         kind: TokenKind::Keyword(Keyword::Int),
    ///         span: Span { start: 0, end: 3 },
    ///     })
    /// );
    /// ```
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        todo!()
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
        // if let Ok(token) = self.consume(TokenKind::Keyword(KeywordKind::SystemOutPrintln)) {
        //     return Ok(token);
        // }

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
    ///
    /// TODO: I don't know how I feel about pushing to integer.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexeme(lexeme: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(lexeme);
        let token = lexer.next_token();

        assert_eq!(
            token,
            Ok(Token {
                kind,
                span: Span {
                    start: 0,
                    end: lexeme.len()
                }
            })
        );
    }

    #[test]
    fn test_eof() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token();

        assert_eq!(
            token,
            Ok(Token {
                kind: TokenKind::Eof,
                span: Span { start: 0, end: 0 }
            })
        )
    }

    #[test]
    fn test_integer_literal() {
        test_lexeme("1", TokenKind::IntegerLiteral(1));
        test_lexeme("42", TokenKind::IntegerLiteral(42));
        test_lexeme("007", TokenKind::IntegerLiteral(7));
    }

    #[test]
    fn test_boolean_literal() {
        test_lexeme("true", TokenKind::BooleanLiteral(true));
        test_lexeme("false", TokenKind::BooleanLiteral(false));
    }

    #[test]
    fn test_identifier() {
        test_lexeme("hello", TokenKind::Identifier(String::from("hello")));
        test_lexeme("staticx", TokenKind::Identifier(String::from("staticx")));
    }

    #[test]
    fn test_keywords() {
        test_lexeme("boolean", TokenKind::Keyword(KeywordKind::Boolean));
        test_lexeme("class", TokenKind::Keyword(KeywordKind::Class));
        test_lexeme("else", TokenKind::Keyword(KeywordKind::Else));
        test_lexeme("if", TokenKind::Keyword(KeywordKind::If));
        test_lexeme("int", TokenKind::Keyword(KeywordKind::Int));
        test_lexeme("main", TokenKind::Keyword(KeywordKind::Main));
        test_lexeme("new", TokenKind::Keyword(KeywordKind::New));
        test_lexeme("public", TokenKind::Keyword(KeywordKind::Public));
        test_lexeme("return", TokenKind::Keyword(KeywordKind::Return));
        test_lexeme("static", TokenKind::Keyword(KeywordKind::Static));
        test_lexeme(
            "System.out.println",
            TokenKind::Keyword(KeywordKind::SystemOutPrintln),
        );
        test_lexeme("this", TokenKind::Keyword(KeywordKind::This));
        test_lexeme("void", TokenKind::Keyword(KeywordKind::Void));
        test_lexeme("while", TokenKind::Keyword(KeywordKind::While));
    }

    #[test]
    fn test_operators() {
        test_lexeme("+", TokenKind::Operator(OperatorKind::Add));
        test_lexeme("-", TokenKind::Operator(OperatorKind::Subtract));
        test_lexeme("*", TokenKind::Operator(OperatorKind::Multiply));
        test_lexeme("=", TokenKind::Operator(OperatorKind::Assign));
        test_lexeme("&&", TokenKind::Operator(OperatorKind::And));
        test_lexeme("<", TokenKind::Operator(OperatorKind::LessThan));
        test_lexeme("!", TokenKind::Operator(OperatorKind::Not));
    }

    #[test]
    fn test_delimiters() {
        test_lexeme("(", TokenKind::Delimiter(DelimeterKind::LeftParenthesis));
        test_lexeme(")", TokenKind::Delimiter(DelimeterKind::RightParenthesis));
        test_lexeme("[", TokenKind::Delimiter(DelimeterKind::LeftBracket));
        test_lexeme("]", TokenKind::Delimiter(DelimeterKind::RightBracket));
        test_lexeme("{", TokenKind::Delimiter(DelimeterKind::LeftBrace));
        test_lexeme("}", TokenKind::Delimiter(DelimeterKind::RightBrace));
        test_lexeme(",", TokenKind::Delimiter(DelimeterKind::Comma));
        test_lexeme(".", TokenKind::Delimiter(DelimeterKind::Dot));
        test_lexeme(";", TokenKind::Delimiter(DelimeterKind::Semicolon));
    }
}
