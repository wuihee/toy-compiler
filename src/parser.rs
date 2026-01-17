//! # Parser
//!
//! This module parses a stream of tokens and constructs an AST.

use std::error::Error;

use crate::{
    ast::Program,
    lexer::token::{Delimiter, Operator, Token},
};

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
        Ok(Program {
            statements: Vec::new(),
        })
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next_token(&mut self) {
        self.position += 1;
    }

    fn parse_program(&mut self) -> bool {
        while self.parse_statement() {}

        if let Some(Token::Eof) = self.peek() {
            return true;
        }

        false
    }

    fn parse_statement(&mut self) -> bool {
        if self.parse_expression() {
            return true;
        };

        let Some(Token::Identifier(_)) = self.peek() else {
            return false;
        };
        self.next_token();

        let Some(Token::Operator(Operator::Equals)) = self.peek() else {
            return false;
        };
        self.next_token();

        self.parse_expression()
    }

    // Expression ::= Term | Term ((+ | -) Term)*
    fn parse_expression(&mut self) -> bool {
        if !self.parse_term() {
            return false;
        };

        while let Some(Token::Operator(Operator::Plus | Operator::Minus)) = self.peek() {
            self.next_token();
            if !self.parse_term() {
                return false;
            };
        }

        true
    }

    // Term ::= Factor | Factor ((* | /) Factor)*
    fn parse_term(&mut self) -> bool {
        if !self.parse_factor() {
            return false;
        };

        while let Some(Token::Operator(Operator::Multiply | Operator::Divide)) = self.peek() {
            self.next_token();
            if !self.parse_factor() {
                return false;
            };
        }

        true
    }

    // Factor ::= INTEGER | IDENTIFIER | "(" Expression ")"
    fn parse_factor(&mut self) -> bool {
        match self.peek() {
            // Match INTEGER | IDENTIFIER
            Some(Token::Integer(_)) | Some(Token::Identifier(_)) => {
                self.next_token();
                return true;
            }

            // Match "(" Expression ")"
            Some(Token::Delimiter(Delimiter::LeftParenthesis)) => {
                self.next_token();

                if !self.parse_expression() {
                    return false;
                }

                match self.peek() {
                    Some(Token::Delimiter(Delimiter::RightParenthesis)) => {
                        self.next_token();
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
