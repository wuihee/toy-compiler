//! Lexer
//!
//! Tokenizes an input program.

use std::error::Error;

use crate::lexer::token::{Delimiter, Operator, Token};

/// Represents the current state of the lexer.
pub struct Lexer<'a> {
    /// The input program to tokenize.
    input: &'a str,

    /// The position of the current token.
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Initialize a new lexer with a program.
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    /// Scans the entire `input`.
    pub fn scan(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            tokens.push(token.clone());

            if matches!(token, Token::Eof) {
                break;
            }
        }

        Ok(tokens)
    }

    /// Retrieves the next token.
    fn next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.skip_whitespace();

        let character = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        if character.is_ascii_digit() {
            let literal = self.next_integer();
            return Ok(Token::Integer(literal));
        }

        if character.is_ascii_alphabetic() || character == '_' {
            let identifier = self.next_identifier();
            return Ok(Token::Identifier(identifier.to_string()));
        }

        let token = match character {
            '+' => Token::Operator(Operator::Plus),
            '-' => Token::Operator(Operator::Minus),
            '*' => Token::Operator(Operator::Multiply),
            '/' => Token::Operator(Operator::Divide),
            '=' => Token::Operator(Operator::Equals),
            '(' => Token::Delimiter(Delimiter::LeftParenthesis),
            ')' => Token::Delimiter(Delimiter::RightParenthesis),
            ';' => Token::Delimiter(Delimiter::Semicolon),
            _ => {
                return Err(format!(
                    "Invalid character '{}' at position {}",
                    character, self.position
                )
                .into());
            }
        };

        self.advance();
        Ok(token)
    }

    /// Peek at the current character without advancing.
    fn peek(&self) -> Option<char> {
        self.input
            .as_bytes()
            .get(self.position)
            .map(|&byte| byte as char)
    }

    /// Advance by one character and return in.
    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.position += 1;
        Some(character)
    }

    /// Move `position` forward past all whitespace.
    fn skip_whitespace(&mut self) {
        while self
            .peek()
            .map_or(false, |character| character.is_whitespace())
        {
            self.advance();
        }
    }

    /// Retrieve the next literal.
    fn next_integer(&mut self) -> i64 {
        let start = self.position;
        while self
            .peek()
            .map_or(false, |character| character.is_ascii_digit())
        {
            self.advance();
        }
        self.input[start..self.position].parse().unwrap()
    }

    /// Retrieve the next identifier.
    fn next_identifier(&mut self) -> &str {
        let start = self.position;

        if let Some(first_character) = self.peek() {
            if first_character.is_ascii_alphabetic() || first_character == '_' {
                self.advance();
            } else {
                return &self.input[start..self.position];
            }
        }

        while self.peek().map_or(false, |character| {
            character.is_ascii_alphanumeric() || character == '_'
        }) {
            self.advance();
        }

        &self.input[start..self.position]
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexer::Lexer,
        token::{Delimiter, Operator, Token},
    };

    #[test]
    fn test_simple_expression() {
        let tokens = Lexer::new("1 + 2;").scan().unwrap();
        let expected = vec![
            Token::Integer(1),
            Token::Operator(Operator::Plus),
            Token::Integer(2),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifiers() {
        let tokens = Lexer::new("abc def _x foo123").scan().unwrap();
        let expected = vec![
            Token::Identifier("abc".into()),
            Token::Identifier("def".into()),
            Token::Identifier("_x".into()),
            Token::Identifier("foo123".into()),
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_literals() {
        let tokens = Lexer::new("42 003 99").scan().unwrap();
        let expected = vec![
            Token::Integer(42),
            Token::Integer(3),
            Token::Integer(99),
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators() {
        let tokens = Lexer::new("+ - * / = ( )").scan().unwrap();
        let expected = vec![
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Equals),
            Token::Delimiter(Delimiter::LeftParenthesis),
            Token::Delimiter(Delimiter::RightParenthesis),
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_weird_spacing() {
        let tokens = Lexer::new("   12   +   34   ").scan().unwrap();
        let expected = vec![
            Token::Integer(12),
            Token::Operator(Operator::Plus),
            Token::Integer(34),
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic]
    fn test_invalid_character() {
        Lexer::new("&").scan().unwrap();
    }
}
