//! # Parser
//!
//! This module parses a stream of tokens and constructs an AST.

use std::error::Error;

use crate::{ast::Program, lexer::token::Token};

/// Represents the current state of a parser.
pub struct Parser {
    /// A sequence of tokens representing a program.
    tokens: Vec<Token>,

    /// The position of the current token.
    position: usize,
}

impl Parser {
    /// Instantiate a new parser with a stream of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Parse the input tokens into an AST.
    ///
    /// # Returns
    ///
    /// `Program` on success which represents the root of the AST, or `Error`
    /// on failure if the syntax of the program is invalid.
    pub fn parse(&mut self) -> Result<Program, Box<dyn Error>> {
        // Each production corresponds to a function.
        // Each function tries to "expand" by calling the next function.
        // At the base case, the function (production) checks if the current token
        // matches, and if it does, moves forward.

        Ok(Program {
            statements: Vec::new(),
        })
    }

    fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position])
        }
    }

    fn next_token(&mut self) {
        self.position += 1;
    }

    fn parse_program(&mut self) -> bool {
        if self.parse_statement() {
            if let Some(Token::Eof) = self.peek() {
                return true;
            }
        }

        false
    }

    fn parse_statement(&mut self) -> bool {
        false
    }

    fn parse_expression(&self) -> bool {
        true
    }

    fn parse_term(&mut self) -> bool {
        if self.parse_factor() {}

        false
    }

    fn parse_factor(&mut self) -> bool {
        match self.peek() {
            Some(Token::Integer(_)) | Some(Token::Identifier(_)) => {
                self.next_token();
                return true;
            }
            Some(_) => return self.parse_expression(),
            None => return false,
        }
    }
}
