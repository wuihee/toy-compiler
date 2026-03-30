//! # Lexer
//!
//! This module is responsible for defining the `Lexer` which converts a given
//! program into a sequnce of tokens.
//!
//! ## Invariants
//!
//! - **Position**: `position` always points to the next unread symbol.
//! - **Longest Valid Token**: The lexer always matches the longest valid token.
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
    token::{Delimiter, Keyword, Operator, Span, Token, TokenKind},
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
        // Skip whitespace.
        while let Some(' ') = self.peek() {
            self.position += 1;
        }

        // Return Eof token if there are no symbols left to read.
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

        // TODO: Deal with comments.
        match symbol {
            'a'..='z' | 'A'..='Z' => self.next_identifier(),
            '0'..='9' => self.next_integer(),
            '+' => self.consume(TokenKind::Operator(Operator::Add)),
            '-' => self.consume(TokenKind::Operator(Operator::Subtract)),
            '*' => self.consume(TokenKind::Operator(Operator::Multiply)),
            '=' => self.consume(TokenKind::Operator(Operator::Assign)),
            '!' => self.consume(TokenKind::Operator(Operator::Not)),
            '<' => self.consume(TokenKind::Operator(Operator::LessThan)),
            '&' => self.consume(TokenKind::Operator(Operator::And)),
            '(' => self.consume(TokenKind::Delimiter(Delimiter::LeftParenthesis)),
            ')' => self.consume(TokenKind::Delimiter(Delimiter::RightParenthesis)),
            '[' => self.consume(TokenKind::Delimiter(Delimiter::LeftBracket)),
            ']' => self.consume(TokenKind::Delimiter(Delimiter::RightBracket)),
            '{' => self.consume(TokenKind::Delimiter(Delimiter::LeftBrace)),
            '}' => self.consume(TokenKind::Delimiter(Delimiter::RightBrace)),
            ',' => self.consume(TokenKind::Delimiter(Delimiter::Comma)),
            '.' => self.consume(TokenKind::Delimiter(Delimiter::Dot)),
            ';' => self.consume(TokenKind::Delimiter(Delimiter::Semicolon)),

            // The current `symbol` is unrecognized in the language.
            symbol => {
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
        if let Ok(token) = self.consume(TokenKind::Keyword(Keyword::SystemOutPrintln)) {
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

    /// Consume the next lexeme as token `kind` if available.
    ///
    /// # Arguments
    ///
    /// - `kind`: Check that the next token is of this kind.
    fn consume(&mut self, kind: TokenKind) -> Result<Token, LexerError> {
        let Some(lexeme) = kind.lexeme() else {
            panic!("consume() called on a TokenKind without a fixed lexeme: {kind:?}");
        };

        let length = lexeme.len();
        let start = self.position;
        let end = start + length;

        if let Some(slice) = self.source.get(start..end)
            && slice == lexeme
        {
            self.position += length;
            return Ok(Token {
                kind,
                span: Span { start, end },
            });
        } else {
            Err(LexerError::UnexpectedToken {
                expected: kind,
                span: Span { start, end },
            })
        }
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
        test_lexeme("boolean", TokenKind::Keyword(Keyword::Boolean));
        test_lexeme("class", TokenKind::Keyword(Keyword::Class));
        test_lexeme("else", TokenKind::Keyword(Keyword::Else));
        test_lexeme("if", TokenKind::Keyword(Keyword::If));
        test_lexeme("int", TokenKind::Keyword(Keyword::Int));
        test_lexeme("main", TokenKind::Keyword(Keyword::Main));
        test_lexeme("new", TokenKind::Keyword(Keyword::New));
        test_lexeme("public", TokenKind::Keyword(Keyword::Public));
        test_lexeme("return", TokenKind::Keyword(Keyword::Return));
        test_lexeme("static", TokenKind::Keyword(Keyword::Static));
        test_lexeme(
            "System.out.println",
            TokenKind::Keyword(Keyword::SystemOutPrintln),
        );
        test_lexeme("this", TokenKind::Keyword(Keyword::This));
        test_lexeme("void", TokenKind::Keyword(Keyword::Void));
        test_lexeme("while", TokenKind::Keyword(Keyword::While));
    }

    #[test]
    fn test_operators() {
        test_lexeme("+", TokenKind::Operator(Operator::Add));
        test_lexeme("-", TokenKind::Operator(Operator::Subtract));
        test_lexeme("*", TokenKind::Operator(Operator::Multiply));
        test_lexeme("=", TokenKind::Operator(Operator::Assign));
        test_lexeme("&&", TokenKind::Operator(Operator::And));
        test_lexeme("<", TokenKind::Operator(Operator::LessThan));
        test_lexeme("!", TokenKind::Operator(Operator::Not));
    }

    #[test]
    fn test_delimiters() {
        test_lexeme("(", TokenKind::Delimiter(Delimiter::LeftParenthesis));
        test_lexeme(")", TokenKind::Delimiter(Delimiter::RightParenthesis));
        test_lexeme("[", TokenKind::Delimiter(Delimiter::LeftBracket));
        test_lexeme("]", TokenKind::Delimiter(Delimiter::RightBracket));
        test_lexeme("{", TokenKind::Delimiter(Delimiter::LeftBrace));
        test_lexeme("}", TokenKind::Delimiter(Delimiter::RightBrace));
        test_lexeme(",", TokenKind::Delimiter(Delimiter::Comma));
        test_lexeme(".", TokenKind::Delimiter(Delimiter::Dot));
        test_lexeme(";", TokenKind::Delimiter(Delimiter::Semicolon));
    }
}
