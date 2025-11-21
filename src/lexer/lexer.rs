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

        if let Some(character) = self.get_current_character() {
            if character.is_ascii_alphanumeric() {
                self.next_literal();
            }

            if character.is_numeric() {
                self.next_identifier();
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
                _ => return Err(format!("Invalid").into()),
            };
        }

        Err(format!("Invalid").into())
    }

    /// Retrieve the next literal.
    fn next_literal(&mut self) {}

    /// Retrieve the next identifier.
    fn next_identifier(&mut self) {}

    /// Move `position` forward past all whitespace.
    fn skip_whitespace(&mut self) {
        while let Some(character) = self.get_current_character() {
            if character.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    /// Retrieve the current character of the `input`.
    fn get_current_character(&self) -> Option<char> {
        self.input
            .as_bytes()
            .get(self.position)
            .map(|byte| *byte as char)
    }
}
