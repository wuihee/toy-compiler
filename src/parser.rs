//! # Parser
//!
//! This module takes a stream of [`Token`]s, and transforms them into an AST.
//!
//! ## Resources
//!
//! - [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)

mod binding_powers;
pub mod errors;

use std::collections::VecDeque;

use crate::{
    ast::{Class, Expression, Identifier, MainClass, Method, Program, Statement, Type, Variable},
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
    parser::errors::ParseError,
};

/// This struct transforms a stream of [`Token`]s into an AST.
pub struct Parser<'a> {
    /// A [`Lexer`] containing the stream of [`Token`]s from program.
    lexer: Lexer<'a>,

    /// When we peek or consume, fill this buffer from `lexer`.
    lookahead: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    /// Instantiate a new [`Parser`].
    ///
    /// # Arguments
    ///
    /// - `lexer`: A [`Lexer`] holding the [`Token`]s from a MiniJava program.
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            lookahead: VecDeque::new(),
        }
    }

    /// Transforms a stream of [`Token`]s into an AST with a root of [`Program`].
    ///
    /// # Example
    ///
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        // Pre: lexer.peek() in FIRST(P).
        // Post: All tokens in P consumed.

        let main = self.parse_main_class()?;

        let mut classes = Vec::<Class>::new();

        // TODO: Should I be using self.lexer.peek()?
        while let Ok(class) = self.parse_class() {
            classes.push(class);
        }

        Ok(Program { main, classes })
    }

    fn parse_main_class(&mut self) -> Result<MainClass, ParseError> {
        self.eat(TokenKind::Class)?;
        let name = self.eat_identifier()?;
        self.eat(TokenKind::LeftBrace)?;
        self.eat(TokenKind::Public)?;
        self.eat(TokenKind::Static)?;
        self.eat(TokenKind::Void)?;
        self.eat(TokenKind::Main)?;
        self.eat(TokenKind::LeftParenthesis)?;
        self.eat(TokenKind::String)?;
        self.eat(TokenKind::LeftBracket)?;
        self.eat(TokenKind::RightBracket)?;
        self.eat_identifier()?;
        self.eat(TokenKind::RightParenthesis)?;
        self.eat(TokenKind::LeftBrace)?;
        let body = self.parse_statement()?;
        self.eat(TokenKind::RightBrace)?;
        self.eat(TokenKind::RightBrace)?;

        Ok(MainClass { name, body })
    }

    fn parse_class(&mut self) -> Result<Class, ParseError> {
        self.eat(TokenKind::Class)?;
        let name = self.eat_identifier()?;

        let mut super_class = None;
        if self.eat(TokenKind::Extends).is_ok() {
            super_class = Some(self.eat_identifier()?);
        }

        self.eat(TokenKind::LeftBrace)?;

        let mut fields = Vec::<Variable>::new();
        while let Ok(field) = self.parse_variable() {
            fields.push(field);
        }

        let mut methods = Vec::<Method>::new();
        while let Ok(method) = self.parse_method() {
            methods.push(method);
        }

        self.eat(TokenKind::RightBrace)?;

        Ok(Class {
            name,
            super_class,
            fields,
            methods,
        })
    }

    fn parse_method(&mut self) -> Result<Method, ParseError> {
        self.eat(TokenKind::Public)?;
        let return_type = self.parse_type()?;
        let name = self.eat_identifier()?;
        self.eat(TokenKind::LeftParenthesis)?;

        // Method parameters.
        let mut parameters = Vec::<Variable>::new();
        if self.peek_next().kind != TokenKind::RightParenthesis {
            loop {
                let parameter_type = self.parse_type()?;
                let parameter_name = self.eat_identifier()?;
                let parameter = Variable {
                    ty: parameter_type,
                    name: parameter_name,
                };
                parameters.push(parameter);

                if self.eat(TokenKind::Comma).is_err() {
                    break;
                }
            }
        }

        self.eat(TokenKind::RightParenthesis)?;
        self.eat(TokenKind::LeftBrace)?;

        // Variable declarations.
        let mut variables = Vec::<Variable>::new();
        while matches!(
            self.peek_next().kind,
            TokenKind::Int | TokenKind::Boolean | TokenKind::Identifier(_)
        ) && matches!(self.peek(1).kind, TokenKind::Identifier(_))
        {
            variables.push(self.parse_variable()?);
        }

        // Method body.
        let mut body = Vec::<Statement>::new();
        while self.peek_next().kind != TokenKind::Return {
            body.push(self.parse_statement()?);
        }

        // Return.
        self.eat(TokenKind::Return)?;
        let return_expression = self.parse_expression()?;
        self.eat(TokenKind::Semicolon)?;
        self.eat(TokenKind::RightBrace)?;

        Ok(Method {
            return_type,
            name,
            parameters,
            variables,
            body,
            return_expression,
        })
    }

    fn parse_variable(&mut self) -> Result<Variable, ParseError> {
        let ty = self.parse_type()?;
        let name = self.eat_identifier()?;
        self.eat(TokenKind::Semicolon)?;

        Ok(Variable { ty, name })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let token = self.peek_next();

        Ok(match token.kind {
            // `boolean`
            TokenKind::Boolean => {
                self.eat_next();
                Type::Boolean
            }

            TokenKind::Int => {
                self.eat_next();

                match self.peek_next().kind {
                    // `int[]`
                    TokenKind::LeftBracket => {
                        self.eat(TokenKind::LeftBracket)?;
                        self.eat(TokenKind::RightBracket)?;

                        Type::IntegerArray
                    }

                    // `int`
                    _ => Type::Integer,
                }
            }

            // Some class name. E.g. `Foo`.
            TokenKind::Identifier(_) => {
                let identifier = self.eat_identifier()?;

                Type::Identifier(identifier)
            }

            _ => {
                return Err(ParseError::UnexpectedToken {
                    kind: token.kind.clone(),
                    span: token.span.clone(),
                });
            }
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.peek_next();

        match &token.kind {
            // "{" ( Statement )* "}"
            TokenKind::LeftBrace => {
                self.eat(TokenKind::LeftBrace)?;

                let mut statements = Vec::new();

                // Parse statements until we see a closing '}'.
                while self.peek_next().kind != TokenKind::RightBrace {
                    statements.push(self.parse_statement()?);
                }

                self.eat(TokenKind::RightBrace)?;

                Ok(Statement::Block { statements })
            }

            // "if" "(" Expression ")" Statement "else" Statement
            TokenKind::If => {
                self.eat(TokenKind::If)?;
                self.eat(TokenKind::LeftParenthesis)?;
                let condition = self.parse_expression()?;
                self.eat(TokenKind::RightParenthesis)?;
                let then_branch = Box::new(self.parse_statement()?);
                self.eat(TokenKind::Else)?;
                let else_branch = Box::new(self.parse_statement()?);

                Ok(Statement::If {
                    condition,
                    then_branch,
                    else_branch,
                })
            }

            // "while" "(" Expression ")" Statement
            TokenKind::While => {
                self.eat(TokenKind::While)?;
                self.eat(TokenKind::LeftParenthesis)?;
                let condition = self.parse_expression()?;
                self.eat(TokenKind::RightParenthesis)?;
                let body = Box::new(self.parse_statement()?);

                Ok(Statement::While { condition, body })
            }

            // "System.out.println" "(" Expression ")" ";"
            TokenKind::SystemOutPrintln => {
                self.eat(TokenKind::SystemOutPrintln)?;
                self.eat(TokenKind::LeftParenthesis)?;
                let expression = self.parse_expression()?;
                self.eat(TokenKind::RightParenthesis)?;
                self.eat(TokenKind::Semicolon)?;

                Ok(Statement::Print { expression })
            }

            TokenKind::Identifier(_) => {
                let identifier = self.eat_identifier()?;
                let token = self.eat_next();

                match token.kind {
                    // Identifier "=" Expression ";"
                    TokenKind::Equal => {
                        let value = self.parse_expression()?;
                        self.eat(TokenKind::Semicolon)?;

                        Ok(Statement::Assign {
                            target: identifier,
                            value,
                        })
                    }

                    // Identifier "[" Expression "]" "=" Expression ";"
                    TokenKind::LeftBracket => {
                        let index = self.parse_expression()?;
                        self.eat(TokenKind::RightBracket)?;
                        self.eat(TokenKind::Equal)?;
                        let value = self.parse_expression()?;
                        self.eat(TokenKind::Semicolon)?;

                        Ok(Statement::ArrayAssign {
                            array: identifier,
                            index,
                            value,
                        })
                    }

                    _ => Err(ParseError::UnexpectedToken {
                        kind: token.kind.clone(),
                        span: token.span.clone(),
                    }),
                }
            }

            _ => Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
                span: token.span.clone(),
            }),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_expression_bp(0)
    }

    /// Handles expressions that require precedence with Pratt Parsing.
    ///
    /// Our goal is to build an AST - concretely, this will be an [`Expression']. The core of this
    /// algorithm are assigning binding powers to operators which determine their precedence. A
    /// token will get "pulled" towards the operator with a higher binding power when building the
    /// AST.
    ///
    /// 1. Initialize LHS. We could encounter a prefix operator here, but we simply build our
    ///    tree and recurse.
    /// 2. Start looping and processing operators.
    /// 3. If the LHS is getting pulled towards the operator, we consume it, update the tree
    ///    (recursing) if necessary, and repeat.
    /// 4. If we're getting pulled away from the operator, return our current tree (i.e. `lhs`).
    ///    In the big picture, this means we've completed building the subtree that will likely
    ///    end up as the right subtree of the previous frame which called us.
    fn parse_expression_bp(&mut self, min_bp: u8) -> Result<Expression, ParseError> {
        let token = self.eat_next();

        let mut lhs = match token.kind {
            TokenKind::IntegerLiteral(value) => Expression::IntegerLiteral(value),
            TokenKind::BooleanLiteral(value) => Expression::BooleanLiteral(value),
            TokenKind::Identifier(value) => Expression::Identifier(Identifier::new(value)),
            TokenKind::This => Expression::This,

            TokenKind::New => {
                match self.peek_next().kind {
                    // "new" "int" "[" Expression "]"
                    TokenKind::Int => {
                        self.eat(TokenKind::Int)?;
                        self.eat(TokenKind::LeftBracket)?;

                        // Binding power of 0 because this is array instantiation.
                        let length = Box::new(self.parse_expression_bp(0)?);
                        self.eat(TokenKind::RightBracket)?;

                        Expression::NewArray { length }
                    }

                    // "new" Identifier "(" ")"
                    _ => {
                        let identifier = self.eat_identifier()?;
                        self.eat(TokenKind::LeftParenthesis)?;
                        self.eat(TokenKind::RightParenthesis)?;

                        Expression::NewObject { name: identifier }
                    }
                }
            }

            // "!" Expression
            TokenKind::Bang => {
                let ((), right_bp) = binding_powers::prefix_binding_power(&TokenKind::Bang)
                    .unwrap_or_else(|| unreachable!("Bang is always a valid prefix operator"));
                let operand = Box::new(self.parse_expression_bp(right_bp)?);

                Expression::Not { operand }
            }

            // "(" Expression ")"
            TokenKind::LeftParenthesis => {
                let expression = self.parse_expression_bp(0)?;
                self.eat(TokenKind::RightParenthesis)?;

                expression
            }

            kind => {
                return Err(ParseError::UnexpectedToken {
                    kind,
                    span: token.span.clone(),
                });
            }
        };

        loop {
            let token = self.peek_next();
            let operator = token.kind.clone();
            let span = token.span.clone();

            // Check if the next token is a postfix operator.
            if let Some((left_bp, ())) = binding_powers::postfix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }

                // Consume operator.
                self.eat_next();

                lhs = match operator {
                    TokenKind::Dot => {
                        let token = self.peek_next();
                        let kind = token.kind.clone();
                        let span = token.span.clone();

                        match kind {
                            // Expression "." "length"
                            TokenKind::Length => {
                                self.eat(TokenKind::Length)?;

                                Expression::ArrayLength {
                                    array: Box::new(lhs),
                                }
                            }

                            TokenKind::Identifier(name) => {
                                let method = Identifier::new(name);

                                self.eat(TokenKind::LeftParenthesis)?;

                                let mut args = Vec::<Expression>::new();

                                while self.peek_next().kind != TokenKind::RightParenthesis {
                                    let arg = self.parse_expression_bp(0)?;
                                    args.push(arg);

                                    self.eat(TokenKind::Comma)?;
                                }

                                self.eat(TokenKind::RightParenthesis)?;

                                Expression::Call {
                                    receiver: Box::new(lhs),
                                    method,
                                    args,
                                }
                            }

                            _ => return Err(ParseError::UnexpectedToken { kind, span }),
                        }
                    }

                    // Build expression for `array[index]`.
                    TokenKind::LeftBracket => {
                        let integer = self.eat_integer()?;
                        let index = Box::new(Expression::IntegerLiteral(integer));

                        self.eat(TokenKind::RightBracket)?;

                        Expression::ArrayLookup {
                            array: Box::new(lhs),
                            index,
                        }
                    }

                    // Token is not a valid postfix operator.
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            kind: operator,
                            span,
                        });
                    }
                };

                continue;
            }

            // Handle infix operator.
            if let Some((left_bp, right_bp)) = binding_powers::infix_binding_power(&operator) {
                if left_bp < min_bp {
                    break;
                }

                // Consume operator.
                self.eat_next();

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
                        return Err(ParseError::UnexpectedToken {
                            kind: operator,
                            span,
                        });
                    }
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    /// Ensures there are `n` tokens in `lookahead`.
    ///
    /// This method is a layer of abstraction between the `lexer` and token consumption in the
    /// parser, and should be the only method calling `self.lexer.next()`.
    pub fn fill(&mut self, n: usize) {
        while self.lookahead.len() < n {
            let token = self.lexer.next_token();
            self.lookahead.push_back(token);
        }
    }

    /// Peek the nth token.
    fn peek(&mut self, n: usize) -> &Token {
        self.fill(n + 1);
        self.lookahead
            .get(n)
            .expect("`fill(n + 1)` guarantees at least `n + 1` tokens in `lookahead`.")
    }

    /// Peek at the next token.
    fn peek_next(&mut self) -> &Token {
        self.peek(0)
    }

    /// Consume the next token.
    fn eat_next(&mut self) -> Token {
        self.fill(1);
        self.lookahead
            .pop_front()
            .expect("`fill(1)` guarantees at least 1 token in `lookahead`.")
    }

    /// Checks that the next token matches `kind`, and consume it.
    fn eat(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let token = self.peek_next();

        if token.kind == kind {
            Ok(self.eat_next())
        } else {
            Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
                span: token.span.clone(),
            })
        }
    }

    /// Checks that the next token is [`TokenKind::Identifier`], consume it, and return the
    /// identifer `String`.
    fn eat_identifier(&mut self) -> Result<Identifier, ParseError> {
        let token = self.peek_next();

        if let TokenKind::Identifier(identifier) = &token.kind {
            let identifier = identifier.to_string();
            self.eat_next();
            Ok(Identifier(identifier))
        } else {
            Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
                span: token.span.clone(),
            })
        }
    }

    /// Checks that the next token is [`TokenKind::IntegerLiteral`], consume it, and return the
    /// integer `i64`.
    fn eat_integer(&mut self) -> Result<i64, ParseError> {
        let token = self.peek_next();

        if let TokenKind::IntegerLiteral(integer) = &token.kind {
            let integer = *integer;
            self.eat_next();
            Ok(integer)
        } else {
            Err(ParseError::UnexpectedToken {
                kind: token.kind.clone(),
                span: token.span.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use indoc::indoc;

    use super::*;

    impl From<i64> for Expression {
        fn from(value: i64) -> Self {
            Expression::IntegerLiteral(value)
        }
    }

    impl From<bool> for Expression {
        fn from(value: bool) -> Self {
            Expression::BooleanLiteral(value)
        }
    }

    impl From<&str> for Expression {
        fn from(value: &str) -> Self {
            Expression::Identifier(Identifier::new(value))
        }
    }

    macro_rules! binary_expressions {
        ($($method:ident => $variant:ident),* $(,)?) => {
            $(fn $method(left: impl Into<Expression>, right: impl Into<Expression>) -> Expression {
                Expression::$variant {
                    left: Box::new(left.into()),
                    right: Box::new(right.into()),
                }
            })*
        };
    }

    binary_expressions! {
        plus => Plus,
        minus => Minus,
        times => Times,
        less_than => LessThan,
        and => And,
    }

    fn variable(ty: Type, name: &str) -> Variable {
        Variable {
            ty,
            name: Identifier::new(name),
        }
    }

    fn block(statements: impl Into<Vec<Statement>>) -> Statement {
        Statement::Block {
            statements: statements.into(),
        }
    }

    fn if_else(
        condition: bool,
        if_branch: impl Into<Vec<Statement>>,
        else_branch: impl Into<Vec<Statement>>,
    ) -> Statement {
        Statement::If {
            condition: Expression::BooleanLiteral(condition),
            then_branch: Box::new(block(if_branch)),
            else_branch: Box::new(block(else_branch)),
        }
    }

    fn while_loop(
        value: impl Into<Expression>,
        statements: impl Into<Vec<Statement>>,
    ) -> Statement {
        Statement::While {
            condition: value.into(),
            body: Box::new(block(statements)),
        }
    }

    fn println(value: impl Into<Expression>) -> Statement {
        Statement::Print {
            expression: value.into(),
        }
    }

    fn assign(target: &str, value: impl Into<Expression>) -> Statement {
        Statement::Assign {
            target: Identifier::new(target),
            value: value.into(),
        }
    }

    fn array_assign(array: &str, index: i64, value: impl Into<Expression>) -> Statement {
        Statement::ArrayAssign {
            array: Identifier::new(array),
            index: index.into(),
            value: value.into(),
        }
    }

    fn int(value: i64) -> Expression {
        Expression::IntegerLiteral(value)
    }

    fn boolean(value: bool) -> Expression {
        Expression::BooleanLiteral(value)
    }

    fn identifier(value: &str) -> Expression {
        Expression::Identifier(Identifier::new(value))
    }

    fn array_lookup(array: &str, index: i64) -> Expression {
        Expression::ArrayLookup {
            array: Box::new(array.into()),
            index: Box::new(index.into()),
        }
    }

    fn array_length(array: &str) -> Expression {
        Expression::ArrayLength {
            array: Box::new(array.into()),
        }
    }

    fn new_array(length: impl Into<Expression>) -> Expression {
        Expression::NewArray {
            length: Box::new(length.into()),
        }
    }

    fn new_object(name: &str) -> Expression {
        Expression::NewObject {
            name: Identifier::new(name),
        }
    }

    fn not(operand: impl Into<Expression>) -> Expression {
        Expression::Not {
            operand: Box::new(operand.into()),
        }
    }

    /// Tests that a list of test cases of `(source, expected)` parse correctly.
    fn assert_parse<'s, T: Debug + PartialEq>(
        cases: impl IntoIterator<Item = (&'s str, T)>,
        parse: impl Fn(&mut Parser<'s>) -> Result<T, ParseError>,
    ) {
        for (source, expected) in cases {
            let mut parser = Parser::new(Lexer::new(source));

            match parse(&mut parser) {
                Ok(value) => {
                    assert_eq!(value, expected, "source: {source}");
                    assert_eq!(
                        parser.peek_next().kind,
                        TokenKind::Eof,
                        "Unconsumed input in {source:?}"
                    );
                }
                Err(error) => panic!("{}", errors::format_error(source, &error)),
            }
        }
    }

    #[test]
    fn main_class() {
        let source = indoc! {"
            class Main {
                public static void main(String[] args) {
                    System.out.println(1);
                }
            }
        "};
        let expected = MainClass {
            name: Identifier::new("Main"),
            body: Statement::Print {
                expression: Expression::IntegerLiteral(1),
            },
        };
        let cases = [(source, expected)];

        assert_parse(cases, Parser::parse_main_class);
    }

    #[test]
    fn class_declaration() {
        let source = "class Foo {}";
        let expected = Class {
            name: Identifier::new("Foo"),
            super_class: None,
            fields: Vec::new(),
            methods: Vec::new(),
        };
        let cases = [(source, expected)];

        assert_parse(cases, Parser::parse_class);
    }

    #[test]
    fn method_declaration() {
        let source = indoc! {"
            public int foo(int x, boolean y) {
                int a;
                int b;

                a = 0;
                b = 1;

                System.out.println(a);
                System.out.println(b);

                return 1;
            }
        "};
        let expected = Method {
            return_type: Type::Integer,
            name: Identifier::new("foo"),
            parameters: vec![variable(Type::Integer, "x"), variable(Type::Boolean, "y")],
            variables: vec![variable(Type::Integer, "a"), variable(Type::Integer, "b")],
            body: vec![assign("a", 0), assign("b", 1), println("a"), println("b")],
            return_expression: int(1),
        };
        let cases = [(source, expected)];

        assert_parse(cases, Parser::parse_method);
    }

    #[test]
    fn variable_declaration() {
        let cases = [
            ("boolean foo;", variable(Type::Boolean, "foo")),
            ("int foo;", variable(Type::Integer, "foo")),
            ("int[] foo;", variable(Type::IntegerArray, "foo")),
            (
                "Foo foo;",
                variable(Type::Identifier(Identifier::new("Foo")), "foo"),
            ),
        ];

        assert_parse(cases, Parser::parse_variable);
    }

    #[test]
    fn ty() {
        let cases = [
            ("boolean", Type::Boolean),
            ("int", Type::Integer),
            ("int[]", Type::IntegerArray),
            ("Foo", Type::Identifier(Identifier::new("Foo"))),
        ];

        assert_parse(cases, Parser::parse_type);
    }

    #[test]
    fn statement() {
        let cases = [
            (
                indoc! {"
                    {
                        System.out.println(0);
                        System.out.println(1);
                        System.out.println(2);
                    }
                "},
                block([println(0), println(1), println(2)]),
            ),
            (
                indoc! {"
                    if (true) {
                        System.out.println(1);
                    } else {
                        foo = 0;
                    }
                "},
                if_else(true, [println(1)], [assign("foo", 0)]),
            ),
            (
                indoc! {"
                    while (true) {
                        System.out.println(1);
                    }
                "},
                while_loop(true, [println(1)]),
            ),
            ("System.out.println(1);", println(1)),
            ("foo = 1;", assign("foo", 1)),
            ("array[0] = 1;", array_assign("array", 0, 1)),
        ];

        assert_parse(cases, Parser::parse_statement);
    }

    #[test]
    fn basic_expression() {
        let cases = [
            ("1 + 1", plus(1, 1)),
            ("1 - 1", minus(1, 1)),
            ("1 * 1", times(1, 1)),
            ("1 < 1", less_than(1, 1)),
            ("true && false", and(true, false)),
            ("array[0]", array_lookup("array", 0)),
            ("array.length", array_length("array")),
            ("1", int(1)),
            ("true", boolean(true)),
            ("false", boolean(false)),
            ("Foo", identifier("Foo")),
            ("this", Expression::This),
            ("new int[10]", new_array(10)),
            ("new Foo()", new_object("Foo")),
            ("!true", not(true)),
            ("(true)", boolean(true)),
        ];

        assert_parse(cases, Parser::parse_expression);
    }
}
