//! # Parser
//!
//! This module parses a stream of tokens and constructs an AST.

use std::{error::Error, fmt};

use crate::{
    ast::{Expression, Identifier, Program, Statement},
    lexer::token::{Delimiter, Operator, Token},
};

// TODO: Include more details.
#[derive(Debug)]
pub struct ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fuck your shit")
    }
}

impl Error for ParserError {}

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
        // Input: Tokens
        // Output: AST
        // Options:
        //  - Pass it through params.
        //  -
        self.parse_program();

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

    // Statement* EOF
    fn parse_program(&mut self) -> Result<Program, Box<dyn Error>> {
        let mut statements = Vec::new();
        while let Ok(statement) = self.parse_statement() {
            statements.push(statement);
        }

        if let Some(Token::Eof) = self.peek() {
            return Ok(Program { statements });
        }

        Err(Box::new(ParserError {}))
    }

    // IDENTIFIER = Expression; | Expression;
    fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        if let Ok(expression) = self.parse_expression() {
            return Ok(Statement::Expression { value: expression });
        };

        let Some(Token::Identifier(identifier)) = self.peek() else {
            return Err(Box::new(ParserError {}));
        };
        let identifier = identifier.clone();
        self.next_token();

        let Some(Token::Operator(Operator::Equals)) = self.peek() else {
            return Err(Box::new(ParserError {}));
        };
        self.next_token();

        let expression = self.parse_expression()?;
        return Ok(Statement::Assignment {
            name: String::from(identifier),
            value: expression,
        });
    }

    // Expression ::= Term | Term ((+ | -) Term)*
    fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
        let left = self.parse_term()?;

        while let Some(operator @ Token::Operator(Operator::Plus | Operator::Minus)) = self.peek() {
            self.next_token();
            self.parse_term()?;
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
    fn parse_factor(&mut self) -> Result<Expression, Box<dyn Error>> {
        match self.peek() {
            Some(Token::Integer(integer)) => {
                let integer = integer.clone();
                self.next_token();
                return Ok(Expression::Integer(integer));
            }

            Some(Token::Identifier(identifier)) => {
                let identifier = identifier.clone();
                self.next_token();
                return Ok(Expression::Identifier(identifier));
            }

            // Match "(" Expression ")"
            Some(Token::Delimiter(Delimiter::LeftParenthesis)) => {
                self.next_token();

                let expression = self.parse_expression()?;

                match self.peek() {
                    Some(Token::Delimiter(Delimiter::RightParenthesis)) => {
                        self.next_token();
                        Ok(expression)
                    }
                    _ => Err(Box::new(ParserError {})),
                }
            }
            _ => Err(Box::new(ParserError {})),
        }
    }
}
