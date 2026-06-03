//! # Parser
//!
//! This module takes a stream of [`Token`]s, and transforms them into an AST.

mod errors;

use std::iter::Peekable;

use crate::{
    ast::{Expression, Identifier, MainClass, Program, Statement},
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::errors::ParseError,
};

/// This struct transforms a stream of [`Token`]s into an AST.
pub struct Parser<'a> {
    /// A [`Lexer`] containing the stream of [`Token`]s from program.
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Instantiate a new [`Parser`].
    ///
    /// # Arguments
    ///
    /// - `lexer`: A [`Lexer`] holding the [`Token`]s from a MiniJava program.
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    /// Transforms a stream of [`Token`]s into an AST with a root of [`Program`].
    ///
    /// # Example
    ///
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        // Pre: lexer.peek() in FIRST(P).
        // Post: All tokens in P consumed.

        let main_class = self.parse_main_class()?;

        todo!()
    }

    fn parse_main_class(&mut self) -> Result<MainClass, ParseError> {
        self.expect(TokenKind::Class)?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        self.expect(TokenKind::Public)?;
        self.expect(TokenKind::Static)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::Main)?;
        self.expect(TokenKind::LeftParenthesis)?;
        self.expect(TokenKind::String)?;
        self.expect(TokenKind::LeftBracket)?;
        self.expect(TokenKind::RightBracket)?;
        self.expect_identifier()?;
        self.expect(TokenKind::RightParenthesis)?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_statement()?;
        self.expect(TokenKind::RightBrace)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(MainClass { name, body })
    }

    fn parse_class(&mut self) {}

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.peek()?;

        match &token.kind {
            // "{" ( Statement )* "}"
            TokenKind::LeftBrace => {
                self.expect(TokenKind::LeftBrace)?;

                let mut statements = Vec::new();

                // Parse statements until we see a closing '}'.
                while self.lexer.peek().map(|token| &token.kind) != Some(&TokenKind::RightBrace) {
                    statements.push(self.parse_statement()?);
                }

                self.expect(TokenKind::RightBrace)?;

                Ok(Statement::Block { statements })
            }

            // "if" "(" Expression ")" Statement "else" Statement
            TokenKind::If => {
                self.expect(TokenKind::If)?;
                self.expect(TokenKind::LeftParenthesis)?;
                let condition = self.parse_expression()?;
                self.expect(TokenKind::RightParenthesis)?;
                let then_branch = Box::new(self.parse_statement()?);
                self.expect(TokenKind::Else)?;
                let else_branch = Box::new(self.parse_statement()?);

                Ok(Statement::If {
                    condition,
                    then_branch,
                    else_branch,
                })
            }

            // "while" "(" Expression ")" Statement
            TokenKind::While => {
                self.expect(TokenKind::While)?;
                self.expect(TokenKind::LeftParenthesis)?;
                let condition = self.parse_expression()?;
                self.expect(TokenKind::RightParenthesis)?;
                let body = Box::new(self.parse_statement()?);

                Ok(Statement::While { condition, body })
            }

            // "System.out.println" "(" Expression ")" ";"
            TokenKind::SystemOutPrintln => {
                self.expect(TokenKind::SystemOutPrintln)?;
                self.expect(TokenKind::LeftParenthesis)?;
                let expression = self.parse_expression()?;
                self.expect(TokenKind::RightParenthesis)?;
                self.expect(TokenKind::Semicolon)?;

                Ok(Statement::Print { expression })
            }

            TokenKind::Identifier(_) => {
                let identifier = self.expect_identifier()?;
                let token = self.peek()?;

                match token.kind {
                    // Identifier "=" Expression ";"
                    TokenKind::Equal => {
                        let value = self.parse_expression()?;
                        self.expect(TokenKind::Semicolon)?;

                        Ok(Statement::Assign {
                            target: identifier,
                            value,
                        })
                    }

                    // Identifier "[" Expression "]" "=" Expression ";"
                    TokenKind::LeftBracket => {
                        self.expect(TokenKind::LeftBracket)?;
                        let index = self.parse_expression()?;
                        self.expect(TokenKind::RightBracket)?;
                        self.expect(TokenKind::Equal)?;
                        let value = self.parse_expression()?;
                        self.expect(TokenKind::Semicolon)?;

                        Ok(Statement::ArrayAssign {
                            array: identifier,
                            index,
                            value,
                        })
                    }

                    _ => Err(ParseError::UnexpectedToken {
                        kind: token.kind.clone(),
                    }),
                }
            }

            _ => Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
            }),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.peek()?;

        match &token.kind {
            // TODO: Precedence!
            // 0-9
            TokenKind::IntegerLiteral(integer) => Ok(Expression::IntegerLiteral(*integer)),

            // "true" | "false"
            TokenKind::BooleanLiteral(boolean) => Ok(Expression::BooleanLiteral(*boolean)),

            // Identifier
            TokenKind::Identifier(identifier) => {
                Ok(Expression::StringLiteral(identifier.to_string()))
            }

            // "this"
            TokenKind::This => Ok(Expression::This),

            TokenKind::New => {
                self.expect(TokenKind::New)?;

                let token = self.peek()?;

                match &token.kind {
                    // "new" "int" "[" Expression "]"
                    TokenKind::Int => {
                        self.expect(TokenKind::Int)?;
                        self.expect(TokenKind::LeftBracket)?;
                        let length = Box::new(self.parse_expression()?);
                        self.expect(TokenKind::RightBracket)?;

                        Ok(Expression::NewArray { length })
                    }

                    // new" Identifier "(" ")"
                    TokenKind::Identifier(_) => {
                        let name = self.expect_identifier()?;
                        self.expect(TokenKind::LeftParenthesis)?;
                        self.expect(TokenKind::RightParenthesis)?;

                        Ok(Expression::NewObject { name })
                    }

                    _ => Err(ParseError::UnexpectedToken {
                        kind: token.kind.clone(),
                    }),
                }
            }

            // "!" Expression
            TokenKind::Bang => {
                self.expect(TokenKind::Bang)?;
                let operand = Box::new(self.parse_expression()?);

                Ok(Expression::Not { operand })
            }

            TokenKind::LeftParenthesis => {
                self.expect(TokenKind::LeftParenthesis)?;
                self.expect(TokenKind::RightParenthesis)?;

                todo!()
            }

            _ => {
                let expression = Box::new(self.parse_expression()?);
                let token = self.peek()?;

                match &token.kind {
                    // Expression "&&" Expression
                    TokenKind::And => {
                        self.expect(TokenKind::And)?;
                        let right = Box::new(self.parse_expression()?);

                        Ok(Expression::And {
                            left: expression,
                            right,
                        })
                    }

                    // Expression "<" Expression
                    TokenKind::LessThan => {
                        self.expect(TokenKind::LessThan)?;
                        let right = Box::new(self.parse_expression()?);

                        Ok(Expression::LessThan {
                            left: expression,
                            right,
                        })
                    }

                    // Expression "+" Expression
                    TokenKind::Plus => {
                        self.expect(TokenKind::Plus)?;
                        let right = Box::new(self.parse_expression()?);

                        Ok(Expression::Plus {
                            left: expression,
                            right,
                        })
                    }

                    // Expression "-" Expression
                    TokenKind::Minus => {
                        self.expect(TokenKind::Minus)?;
                        let right = Box::new(self.parse_expression()?);

                        Ok(Expression::Minus {
                            left: expression,
                            right,
                        })
                    }

                    // Expression "*" Expression
                    TokenKind::Star => {
                        self.expect(TokenKind::Star)?;
                        let right = Box::new(self.parse_expression()?);

                        Ok(Expression::Times {
                            left: expression,
                            right,
                        })
                    }

                    // Expression "[" Expression "]"
                    TokenKind::LeftBracket => {
                        self.expect(TokenKind::LeftBracket)?;
                        let index = Box::new(self.parse_expression()?);
                        self.expect(TokenKind::RightBracket)?;

                        Ok(Expression::ArrayLookup {
                            array: expression,
                            index,
                        })
                    }

                    TokenKind::Dot => {
                        let token = self.peek()?;

                        match &token.kind {
                            // Expression "." "length"
                            TokenKind::Length => Ok(Expression::ArrayLength { array: expression }),

                            // Expression "." Identifier "(" ( Expression ( "," Expression )* )? ")"
                            TokenKind::Identifier(_) => {
                                let method = self.expect_identifier()?;
                                self.expect(TokenKind::LeftParenthesis)?;

                                let mut args = Vec::new();
                                args.push(self.parse_expression()?);
                                while self.lexer.peek().map(|token| &token.kind)
                                    != Some(&TokenKind::RightParenthesis)
                                {
                                    self.expect(TokenKind::Comma)?;
                                    args.push(self.parse_expression()?);
                                }

                                Ok(Expression::Call {
                                    receiver: expression,
                                    method,
                                    args,
                                })
                            }

                            _ => Err(ParseError::UnexpectedToken {
                                kind: token.kind.clone(),
                            }),
                        }
                    }

                    _ => Err(ParseError::UnexpectedToken {
                        kind: token.kind.clone(),
                    }),
                }
            }
        }
    }

    /// Peek at the next token, and return an `UnexpectedEof` if it doesn't exist.
    fn peek(&mut self) -> Result<&Token, ParseError> {
        self.lexer.peek().ok_or(ParseError::UnexpectedEof)
    }

    /// Checks that the next token matches `kind`, and consume it.
    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let token = self.peek()?;

        if token.kind == kind {
            Ok(self.lexer.next().unwrap())
        } else {
            Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
            })
        }
    }

    /// Checks that the next token is [`TokenKind::Identifier`], consumes it, and returns the
    /// identifer `String`.
    fn expect_identifier(&mut self) -> Result<Identifier, ParseError> {
        let token = self.peek()?;

        if let TokenKind::Identifier(identifier) = &token.kind {
            let identifier = identifier.to_string();
            self.lexer.next();
            Ok(Identifier(identifier))
        } else {
            Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_main_class() {
        let source = r#"
            class Main {
                public static void main(String[] args) {
                    System.out.println(1);
                }
            }
            "#;

        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let main = parser.parse_main_class().unwrap();

        assert_eq!(main.name.as_ref(), "Main");
    }
}
