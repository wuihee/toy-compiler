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

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
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
    fn next_token(&mut self) -> Option<Token> {
        // Remove whitespace.
        while matches!(self.peek(), Some(symbol) if symbol.is_whitespace()) {
            self.bump();
        }

        // Remove line comments.
        if matches!(self.peek_by(2), Some(lexeme) if lexeme == "//") {
            self.bump();
            self.bump();

            self.skip_line_comment();
        }

        let start = self.position;

        // Advance to the next symbol.
        let Some(symbol) = self.bump() else {
            return None;
        };

        let token = match symbol {
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
        };

        Some(token)
    }

    /// Get the symbol at the current `position`.
    fn peek(&self) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.position)
            .map(|&symbol| symbol as char)
    }

    /// Peek the next `n` characters.
    fn peek_by(&self, n: usize) -> Option<&str> {
        if self.position + n > self.source.len() {
            return None;
        }

        Some(&self.source[self.position..self.position + n])
    }

    /// Return the symbol at the current `position` and advance.
    fn bump(&mut self) -> Option<char> {
        if let Some(symbol) = self.peek() {
            self.position += 1;
            return Some(symbol);
        }

        None
    }

    /// Skip the next line comment by advancing past the newline.
    fn skip_line_comment(&mut self) {
        while let Some(lexeme) = self.peek_by(2) {
            if lexeme == "\\n" {
                self.bump();
                self.bump();
                break;
            }

            self.bump();
        }
    }

    /// Consume the next keyword or identifier.
    fn consume_identifier(&mut self, start: usize) -> Token {
        while let Some(symbol) = self.peek() {
            if symbol.is_alphanumeric() || symbol == '_' {
                self.bump();
            } else {
                break;
            }
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
        while let Some(symbol) = self.peek() {
            if symbol.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
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

    fn test_lexer(source: &str, kinds: &[TokenKind]) {
        let lexer = Lexer::new(source);

        for (i, token) in lexer.enumerate() {
            // TODO: I'm not testing the span.
            assert_eq!(token, Token::new(kinds[i].clone(), token.span.clone()))
        }
    }

    fn test_lexeme(source: &str, kind: TokenKind) {
        test_lexer(source, &vec![kind]);
    }

    #[test]
    fn simple() {
        let source = "int x = 1 + 2 * 3;";
        let kinds = vec![
            TokenKind::Int,
            TokenKind::Identifier(String::from("x")),
            TokenKind::Equal,
            TokenKind::IntegerLiteral(1),
            TokenKind::Plus,
            TokenKind::IntegerLiteral(2),
            TokenKind::Star,
            TokenKind::IntegerLiteral(3),
            TokenKind::Semicolon,
        ];

        test_lexer(source, &kinds);
    }

    #[test]
    fn eof() {
        let mut lexer = Lexer::new("");
        let token = lexer.next();

        assert_eq!(token, None)
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
        test_lexeme("boolean", TokenKind::Boolean);
        test_lexeme("class", TokenKind::Class);
        test_lexeme("else", TokenKind::Else);
        test_lexeme("extends", TokenKind::Extends);
        test_lexeme("if", TokenKind::If);
        test_lexeme("int", TokenKind::Int);
        test_lexeme("main", TokenKind::Main);
        test_lexeme("new", TokenKind::New);
        test_lexeme("public", TokenKind::Public);
        test_lexeme("return", TokenKind::Return);
        test_lexeme("static", TokenKind::Static);
        test_lexeme("this", TokenKind::This);
        test_lexeme("void", TokenKind::Void);
        test_lexeme("while", TokenKind::While);
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
