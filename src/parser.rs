//! # Parser
//!
//! This module takes a stream of [`Token`]s, and transforms them into an AST.
//!
//! ## Resources
//!
//! - [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)

mod binding_powers;
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

    fn parse_class(&mut self) -> Result<Statement, ParseError> {
        self.expect(TokenKind::Class)?;
        let name = self.expect_identifier()?;

        let mut super_class = None;
        if let Ok(_) = self.expect(TokenKind::Extends) {
            super_class = Some(self.expect_identifier()?);
        }

        self.expect(TokenKind::LeftBrace)?;

        // while self.lexer.pek

        todo!()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.peek()?;

        match &token.kind {
            // "{" ( Statement )* "}"
            TokenKind::LeftBrace => {
                self.expect(TokenKind::LeftBrace)?;

                let mut statements = Vec::new();

                // Parse statements until we see a closing '}'.
                while self.peek()?.kind != TokenKind::RightBrace {
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
        // Note that we are consuming the next token before verifying correctness. This would be
        // a problem if we are backtracking, but our parser does not do that.
        let token = self.lexer.next().ok_or(ParseError::UnexpectedEof)?;

        match &token.kind {
            // 0-9
            TokenKind::IntegerLiteral(integer) => Ok(Expression::IntegerLiteral(*integer)),

            // "true" | "false"
            TokenKind::BooleanLiteral(boolean) => Ok(Expression::BooleanLiteral(*boolean)),

            // Identifier
            TokenKind::Identifier(identifier) => Ok(Expression::Identifier(identifier.to_string())),

            // "this"
            TokenKind::This => Ok(Expression::This),

            // "new"
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

            // "(" Expression ")"
            TokenKind::LeftParenthesis => {
                self.expect(TokenKind::LeftParenthesis)?;
                let expression = Box::new(self.parse_expression()?);
                self.expect(TokenKind::RightParenthesis)?;

                Ok(Expression::Group { expression })
            }

            // Handle expressions with Pratt Parsing.
            _ => self.parse_expression_bp(0),
        }
    }

    /// Handles expressions that require precedence with Pratt Parsing.
    ///
    /// Our goal is to build an AST - concretely, this will be an [`Expression']. The core of this
    /// algorithm are assigning binding powers to operators which determin their precedence. A
    /// token will get "pulled" towards the operator with a higher binding power when building the
    /// AST.
    ///
    /// 1. Initialize LHS. We could encounter a prefix operator here, but we simply build our
    ///    tree and recurse.
    /// 2. Start looping and processing operators.
    /// 3. If we're getting pulled towards the operator, we consume it, update the tree
    ///    (recursing) if necessary, and repeat.
    /// 4. If we're getting pulled away from the operator, return our current tree (i.e. `lhs`).
    ///    In the big picture, this means we've completed building the subtree that will likely
    ///    end up as the right subtree of the previous frame which called us.
    fn parse_expression_bp(&mut self, min_bp: u8) -> Result<Expression, ParseError> {
        let Some(token) = self.lexer.next() else {
            return Err(ParseError::UnexpectedEof);
        };

        let mut lhs = match token.kind {
            TokenKind::IntegerLiteral(value) => Expression::IntegerLiteral(value),
            TokenKind::BooleanLiteral(value) => Expression::BooleanLiteral(value),
            TokenKind::Identifier(value) => Expression::Identifier(value),
            TokenKind::LeftParenthesis => {
                let expression = self.parse_expression_bp(0)?;

                self.expect(TokenKind::RightParenthesis)?;

                expression
            }
            TokenKind::Bang => {
                let ((), right_bp) = binding_powers::prefix_binding_power(&TokenKind::Bang)
                    .unwrap_or_else(|| unreachable!("Bang is always a valid prefix operator"));
                let operand = Box::new(self.parse_expression_bp(right_bp)?);

                Expression::Not { operand }
            }
            kind => return Err(ParseError::UnexpectedToken { kind }),
        };

        loop {
            let token = self.peek()?;
            let operator = token.kind.clone();

            // Check if the next token is a postfix operator.
            if let Some((left_bp, ())) = binding_powers::postfix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }

                // Consume operator.
                self.lexer.next();

                lhs = match operator {
                    // Build expression for `receiver.method(args)`.
                    TokenKind::Dot => {
                        let method = self.expect_identifier()?;
                        self.expect(TokenKind::LeftParenthesis)?;

                        let mut args = Vec::<Expression>::new();

                        while self.peek()?.kind != TokenKind::RightParenthesis {
                            let arg = self.expect_identifier()?.as_str().to_string();
                            args.push(Expression::Identifier(arg));

                            self.expect(TokenKind::Comma)?;
                        }

                        self.expect(TokenKind::RightParenthesis)?;

                        Expression::Call {
                            receiver: Box::new(lhs),
                            method,
                            args,
                        }
                    }

                    // Build expression for `array[index]`.
                    TokenKind::LeftBracket => {
                        let integer = self.expect_integer()?;
                        let index = Box::new(Expression::IntegerLiteral(integer));

                        self.expect(TokenKind::RightBracket)?;

                        Expression::ArrayLookup {
                            array: Box::new(lhs),
                            index,
                        }
                    }

                    // Token is not a valid postfix operator.
                    _ => {
                        return Err(ParseError::UnexpectedToken { kind: operator });
                    }
                };
            }

            // Handle infix operator.
            if let Some((left_bp, right_bp)) = binding_powers::infix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }

                // Consume operator.
                self.lexer.next();

                lhs = match operator {
                    TokenKind::And => {
                        let left = Box::new(lhs);
                        let right = Box::new(self.parse_expression_bp(right_bp)?);

                        Expression::And { left, right }
                    }

                    TokenKind::LessThan => {
                        let left = Box::new(lhs);
                        let right = Box::new(self.parse_expression_bp(right_bp)?);

                        Expression::LessThan { left, right }
                    }

                    TokenKind::Plus => {
                        let left = Box::new(lhs);
                        let right = Box::new(self.parse_expression_bp(right_bp)?);

                        Expression::Plus { left, right }
                    }

                    TokenKind::Minus => {
                        let left = Box::new(lhs);
                        let right = Box::new(self.parse_expression_bp(right_bp)?);

                        Expression::Minus { left, right }
                    }

                    TokenKind::Star => {
                        let left = Box::new(lhs);
                        let right = Box::new(self.parse_expression_bp(right_bp)?);

                        Expression::Times { left, right }
                    }

                    // Token is not a valid infix operator.
                    _ => {
                        return Err(ParseError::UnexpectedToken { kind: operator });
                    }
                }
            }
        }

        Ok(lhs)
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

    /// Checks that the next token is [`TokenKind::Identifier`], consume it, and return the
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

    /// Checks that the next token is [`TokenKind::IntegerLiteral`], consume it, and return the
    /// integer `i64`.
    fn expect_integer(&mut self) -> Result<i64, ParseError> {
        let token = self.peek()?;

        if let TokenKind::IntegerLiteral(integer) = &token.kind {
            let integer = *integer;
            self.lexer.next();
            Ok(integer)
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
        assert_eq!(
            main.body,
            Statement::Print {
                expression: Expression::IntegerLiteral(1)
            }
        )
    }
}
