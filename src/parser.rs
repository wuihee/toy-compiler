//! # Parser
//!
//! This module parses a stream of tokens and constructs an AST.

use std::{error::Error, fmt};

use crate::{
    ast::{BinaryOperator, Expression, Program, Statement},
    lexer::token::{Delimiter, Operator, Token},
};

/// Indicates a parser error.
#[derive(Debug)]
pub struct ParserError {
    /// The parser's error message.
    message: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse: {}", self.message)
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
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let ast = self.parse_program()?;
        Ok(ast)
    }

    /// Move position to point to the next token.
    fn next_token(&mut self) {
        self.position += 1;
    }

    /// Peek at the current token without advancing
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Statement* EOF
    fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Eof) => break,
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
            }
        }

        Ok(Program { statements })
    }

    // IDENTIFIER = Expression;
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // Match IDENTIFIER = Expression;
        if let Some(Token::Identifier(identifier)) = self.peek() {
            let identifier = identifier.clone();
            self.next_token();

            let Some(Token::Operator(Operator::Equals)) = self.peek() else {
                return Err(ParserError {
                    message: String::from("Expected equals"),
                });
            };
            self.next_token();

            let expression = self.parse_expression()?;

            let Some(Token::Delimiter(Delimiter::Semicolon)) = self.peek() else {
                return Err(ParserError {
                    message: String::from("Expected semicolon"),
                });
            };
            self.next_token();

            return Ok(Statement::Assignment {
                name: identifier,
                value: expression,
            });
        }

        Err(ParserError {
            message: String::from("Expected an assignment statement"),
        })
    }

    // Expression ::= Term | Term ((+ | -) Term)*
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_term()?;

        while let Some(Token::Operator(operator @ (Operator::Plus | Operator::Minus))) = self.peek()
        {
            let binary_operator = match operator {
                Operator::Plus => BinaryOperator::Add,
                Operator::Minus => BinaryOperator::Subtract,
                Operator::Multiply => BinaryOperator::Multiply,
                Operator::Divide => BinaryOperator::Divide,
                _ => {
                    return Err(ParserError {
                        message: String::from("Unknown operator while parsing expression"),
                    });
                }
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
    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Operator(operator @ (Operator::Multiply | Operator::Divide))) =
            self.peek()
        {
            let binary_operator = match operator {
                Operator::Plus => BinaryOperator::Add,
                Operator::Minus => BinaryOperator::Subtract,
                Operator::Multiply => BinaryOperator::Multiply,
                Operator::Divide => BinaryOperator::Divide,
                _ => {
                    return Err(ParserError {
                        message: String::from("Unknown operator while parsing term"),
                    });
                }
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
    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
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
                    _ => Err(ParserError {
                        message: String::from("Expected right parenthesis"),
                    }),
                }
            }
            _ => Err(ParserError {
                message: String::from("Failed to match integer, identifier, or (expression)"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{BinaryOperator, Expression, Program, Statement},
        lexer::lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_simple_statement() {
        let tokens = Lexer::new("x = 1;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Integer(1),
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_operation_order() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let tokens = Lexer::new("x = 1 + 2 * 3;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Integer(1)),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Binary {
                        left: Box::new(Expression::Integer(2)),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(Expression::Integer(3)),
                    }),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_multiple_statements() {
        let tokens = Lexer::new("x = 1; y = 2; z = 3;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![
                Statement::Assignment {
                    name: String::from("x"),
                    value: Expression::Integer(1),
                },
                Statement::Assignment {
                    name: String::from("y"),
                    value: Expression::Integer(2),
                },
                Statement::Assignment {
                    name: String::from("z"),
                    value: Expression::Integer(3),
                },
            ],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_addition_subtraction() {
        let tokens = Lexer::new("x = 5 - 3 + 2;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        // Left-associative: (5 - 3) + 2
        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Integer(5)),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(Expression::Integer(3)),
                    }),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Integer(2)),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_multiplication_division() {
        let tokens = Lexer::new("x = 6 / 2 * 3;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        // Left-associative: (6 / 2) * 3
        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Integer(6)),
                        operator: BinaryOperator::Divide,
                        right: Box::new(Expression::Integer(2)),
                    }),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expression::Integer(3)),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parenthesized_expression() {
        // (1 + 2) * 3 - parentheses override precedence
        let tokens = Lexer::new("x = (1 + 2) * 3;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Integer(1)),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Integer(2)),
                    }),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(Expression::Integer(3)),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_nested_parentheses() {
        let tokens = Lexer::new("x = ((1 + 2));").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Integer(1)),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Integer(2)),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_identifier_in_expression() {
        let tokens = Lexer::new("x = y + 1;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Identifier(String::from("y"))),
                    operator: BinaryOperator::Add,
                    right: Box::new(Expression::Integer(1)),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_complex_expression() {
        // a + b * c - d / e
        // Parses as: (a + (b * c)) - (d / e)
        let tokens = Lexer::new("x = a + b * c - d / e;").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program {
            statements: vec![Statement::Assignment {
                name: String::from("x"),
                value: Expression::Binary {
                    left: Box::new(Expression::Binary {
                        left: Box::new(Expression::Identifier(String::from("a"))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Binary {
                            left: Box::new(Expression::Identifier(String::from("b"))),
                            operator: BinaryOperator::Multiply,
                            right: Box::new(Expression::Identifier(String::from("c"))),
                        }),
                    }),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(Expression::Binary {
                        left: Box::new(Expression::Identifier(String::from("d"))),
                        operator: BinaryOperator::Divide,
                        right: Box::new(Expression::Identifier(String::from("e"))),
                    }),
                },
            }],
        };

        assert_eq!(ast, expected);
    }

    #[test]
    #[should_panic(expected = "Expected semicolon")]
    fn test_missing_semicolon() {
        let tokens = Lexer::new("x = 1").scan().unwrap();
        Parser::new(tokens).parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected right parenthesis")]
    fn test_missing_right_parenthesis() {
        let tokens = Lexer::new("x = (1 + 2;").scan().unwrap();
        Parser::new(tokens).parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected equals")]
    fn test_missing_equals() {
        let tokens = Lexer::new("x 1;").scan().unwrap();
        Parser::new(tokens).parse().unwrap();
    }

    #[test]
    fn test_empty_program() {
        let tokens = Lexer::new("").scan().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let expected = Program { statements: vec![] };

        assert_eq!(ast, expected);
    }
}
