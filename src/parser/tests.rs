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

macro_rules! call {
        ($receiver:expr, $method:expr $(, $arg:expr)* $(,)?) => {
            Expression::Call {
                receiver: Box::new($receiver.into()),
                method: Identifier::new($method),
                args: vec![$(Into::<Expression>::into($arg)),*],
            }
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

fn while_loop(value: impl Into<Expression>, statements: impl Into<Vec<Statement>>) -> Statement {
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
fn assert_parses<'s, T: Debug + PartialEq>(
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

/// Parses a list of programs with a provided function.
fn parses<'s, T: Debug + PartialEq>(
    cases: impl IntoIterator<Item = &'s str>,
    parse: impl Fn(&mut Parser<'s>) -> Result<T, ParseError>,
) {
    for source in cases {
        let mut parser = Parser::new(Lexer::new(source));
        parse(&mut parser).unwrap();
    }
}

#[test]
fn parse_variable() {
    let cases = [
        ("int[] foo;", variable(Type::IntegerArray, "foo")),
        ("boolean foo;", variable(Type::Boolean, "foo")),
        ("int foo;", variable(Type::Integer, "foo")),
        (
            "Foo foo;",
            variable(Type::Identifier(Identifier::new("Foo")), "foo"),
        ),
    ];
    assert_parses(cases, Parser::parse_variable);
}

#[test]
#[should_panic]
fn parse_variable_fail() {
    let cases = [
        "String foo;", // Invalid type.
        "int foo",     // No semicolon.
        "int 1;",      // Variable not identifier.
    ];
    parses(cases, Parser::parse_variable);
}
