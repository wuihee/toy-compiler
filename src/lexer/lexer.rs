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
    pub fn from(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    /// Retrieves the next token.
    pub fn next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.skip_whitespace();

        let character = match self.peek() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        if character.is_ascii_digit() {
            let literal = self.next_literal();
            return Ok(Token::Literal(literal.to_string()));
        }

        if character.is_ascii_alphabetic() || character == '_' {
            let identifier = self.next_identifier();
            return Ok(Token::Identifier(identifier.to_string()));
        }

        match character {
            '+' => {
                self.position += 1;
                return Ok(Token::Operator(Operator::Plus));
            }
            '-' => {
                self.position += 1;
                return Ok(Token::Operator(Operator::Minus));
            }
            '*' => {
                self.position += 1;
                return Ok(Token::Operator(Operator::Multiply));
            }
            '/' => {
                self.position += 1;
                return Ok(Token::Operator(Operator::Divide));
            }
            '=' => {
                self.position += 1;
                return Ok(Token::Operator(Operator::Equals));
            }
            '(' => {
                self.position += 1;
                return Ok(Token::Delimiter(Delimiter::LeftParenthesis));
            }
            ')' => {
                self.position += 1;
                return Ok(Token::Delimiter(Delimiter::RightParenthesis));
            }
            _ => {
                return Err(format!(
                    "Invalid character '{}' at position {}",
                    character, self.position
                )
                .into());
            }
        };
    }

    /// Peek at the current character without advancing.
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
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
    fn next_literal(&mut self) -> &str {
        let start = self.position;
        while self
            .peek()
            .map_or(false, |character| character.is_ascii_digit())
        {
            self.advance();
        }
        &self.input[start..self.position]
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
        let input = "1 + 2";
        let mut lexer = Lexer::from(input);

        let tests = vec![
            Token::Literal("1".into()),
            Token::Operator(Operator::Plus),
            Token::Literal("2".into()),
            Token::Eof,
        ];

        for expected in tests {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::from("abc def _x foo123");

        let tests = vec![
            Token::Identifier("abc".into()),
            Token::Identifier("def".into()),
            Token::Identifier("_x".into()),
            Token::Identifier("foo123".into()),
        ];

        for expected in tests {
            assert_eq!(lexer.next_token().unwrap(), expected);
        }
    }

    #[test]
    fn test_literals() {
        let mut lexer = Lexer::from("42 003 99");

        let tests = vec![
            Token::Literal("42".into()),
            Token::Literal("003".into()),
            Token::Literal("99".into()),
            Token::Eof,
        ];

        for expected in tests {
            assert_eq!(lexer.next_token().unwrap(), expected);
        }
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::from("+ - * / = ( )");

        let tests = vec![
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Equals),
            Token::Delimiter(Delimiter::LeftParenthesis),
            Token::Delimiter(Delimiter::RightParenthesis),
            Token::Eof,
        ];

        for expected in tests {
            assert_eq!(lexer.next_token().unwrap(), expected);
        }
    }

    #[test]
    fn test_weird_spacing() {
        let mut lexer = Lexer::from("   12   +   34   ");

        let tests = vec![
            Token::Literal("12".into()),
            Token::Operator(Operator::Plus),
            Token::Literal("34".into()),
            Token::Eof,
        ];

        for expected in tests {
            assert_eq!(lexer.next_token().unwrap(), expected);
        }
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::from("&");

        let result = lexer.next_token();
        assert!(result.is_err());
    }
}
