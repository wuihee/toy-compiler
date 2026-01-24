//! # Parser
//!
//! This module parses a stream of tokens and constructs an AST.

use std::{error::Error, fmt};

use crate::{
    ast::{BinaryOperator, Expression, Program, Statement},
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
        let ast = self.parse_program()?;

        println!("{ast:?}");

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
        // Match IDENTIFIER = Expression;
        if let Some(Token::Identifier(identifier)) = self.peek() {
            let identifier = identifier.clone();
            self.next_token();

            let Some(Token::Operator(Operator::Equals)) = self.peek() else {
                return Err(Box::new(ParserError {}));
            };

            let expression = self.parse_expression()?;

            let Some(Token::Delimiter(Delimiter::Semicolon)) = self.peek() else {
                return Err(Box::new(ParserError {}));
            };

            return Ok(Statement::Assignment {
                name: identifier,
                value: expression,
            });
        }

        // Match Expression;
        let expression = self.parse_expression()?;

        let Some(Token::Delimiter(Delimiter::Semicolon)) = self.peek() else {
            return Err(Box::new(ParserError {}));
        };

        Ok(Statement::Expression { value: expression })
    }

    // Expression ::= Term | Term ((+ | -) Term)*
    fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
        let mut left = self.parse_term()?;

        while let Some(Token::Operator(operator @ (Operator::Plus | Operator::Minus))) = self.peek()
        {
            let binary_operator = match operator {
                Operator::Plus => BinaryOperator::Add,
                Operator::Minus => BinaryOperator::Subtract,
                Operator::Multiply => BinaryOperator::Multiply,
                Operator::Divide => BinaryOperator::Divide,
                _ => return Err(Box::new(ParserError {})),
            };
            self.next_token();

            let right = self.parse_term()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: binary_operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Term ::= Factor | Factor ((* | /) Factor)*
    fn parse_term(&mut self) -> Result<Expression, Box<dyn Error>> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Operator(operator @ (Operator::Multiply | Operator::Divide))) =
            self.peek()
        {
            let binary_operator = match operator {
                Operator::Plus => BinaryOperator::Add,
                Operator::Minus => BinaryOperator::Subtract,
                Operator::Multiply => BinaryOperator::Multiply,
                Operator::Divide => BinaryOperator::Divide,
                _ => return Err(Box::new(ParserError {})),
            };
            self.next_token();

            let right = self.parse_factor()?;

            left = Expression::Binary {
                left: Box::new(left),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        Ok(left)
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
