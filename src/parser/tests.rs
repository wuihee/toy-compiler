#![allow(unused)] // TODO: Remove

use std::fmt::Debug;

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

fn array_lookup(array: impl Into<Expression>, index: impl Into<Expression>) -> Expression {
    Expression::ArrayLookup {
        array: Box::new(array.into()),
        index: Box::new(index.into()),
    }
}

fn array_length(array: impl Into<Expression>) -> Expression {
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

/// Asserts that a list of cases all fail to parse.
fn assert_parses_fails<'s, T: Debug + PartialEq>(
    cases: impl IntoIterator<Item = &'s str>,
    parse: impl Fn(&mut Parser<'s>) -> Result<T, ParseError>,
) {
    for source in cases {
        let mut parser = Parser::new(Lexer::new(source));
        assert!(parse(&mut parser).is_err());
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
fn parse_variable_fail() {
    let cases = [
        "String foo;", // Invalid type.
        "int foo",     // No semicolon.
        "int 1;",      // Variable not identifier.
    ];
    assert_parses_fails(cases, Parser::parse_variable);
}

#[test]
fn parse_type() {
    let cases = [
        ("int[]", Type::IntegerArray),
        ("boolean", Type::Boolean),
        ("int", Type::Integer),
        ("Foo", Type::Identifier(Identifier::new("Foo"))),
    ];
    assert_parses(cases, Parser::parse_type);
}

#[test]
fn parse_type_fail() {
    let cases = [
        "int[",  // Incomplete integer array.
        "int[)", // Wrong brackets.
        "1",     // Invalid type.
    ];
    assert_parses_fails(cases, Parser::parse_type);
}

#[test]
fn parse_expression_basic() {
    let cases = [
        ("1", int(1)),
        ("true", boolean(true)),
        ("false", boolean(false)),
        ("foo", identifier("foo")),
        ("this", Expression::This),
        ("new int[1]", new_array(1)),
        ("new Foo()", new_object("Foo")),
        ("!true", not(true)),
        ("(true)", boolean(true)),
        ("array.length", array_length("array")),
        (
            "Foo.method(arg_1, arg_2)",
            call!("Foo", "method", "arg_1", "arg_2"),
        ),
        ("array.length()", call!("array", "length")),
        ("array[0]", array_lookup("array", 0)),
        ("true && false", and(true, false)),
        ("1 < 2", less_than(1, 2)),
        ("1 + 2", plus(1, 2)),
        ("1 - 2", minus(1, 2)),
        ("1 * 2", times(1, 2)),
    ];
    assert_parses(cases, Parser::parse_expression);
}

#[test]
fn parse_expression_prefix() {
    let cases = [
        ("new int[1 + 1]", new_array(plus(1, 1))),
        (
            "!Foo.is_true(variable)",
            not(call!("Foo", "is_true", "variable")),
        ),
        ("!!true", not(not(true))),
        (
            "!(Foo.is_true(!variable))",
            not(call!("Foo", "is_true", not("variable"))),
        ),
    ];
    assert_parses(cases, Parser::parse_expression);
}

#[test]
fn parse_expression_prefix_fail() {
    let cases = [""];
    assert_parses_fails(cases, Parser::parse_expression);
}

#[test]
fn parse_expression_postfix() {
    let cases = [
        (
            "Foo.get_array(10).length",
            array_length(call!("Foo", "get_array", 10)),
        ),
        (
            "Foo.get_array_maker().get_array(10).length",
            array_length(call!(call!("Foo", "get_array_maker"), "get_array", 10)),
        ),
        (
            "Foo.method(1 + array.length, array.length)",
            call!(
                "Foo",
                "method",
                plus(1, array_length("array")),
                array_length("array")
            ),
        ),
        ("array[1 + 1]", array_lookup("array", plus(1, 1))),
        (
            "Foo.get_array()[1 + 1]",
            array_lookup(call!("Foo", "get_array"), plus(1, 1)),
        ),
    ];
    assert_parses(cases, Parser::parse_expression);
}

#[test]
fn parse_expression_infix() {
    let cases = [
        ("true && false && true", and(and(true, false), true)),
        ("true && 1 < 2", and(true, less_than(1, 2))),
        ("1 < 2 && true", and(less_than(1, 2), true)),
        ("1 < 2 < 3", less_than(less_than(1, 2), 3)),
        ("1 < 2 + 3", less_than(1, plus(2, 3))),
        ("1 + 2 < 3", less_than(plus(1, 2), 3)),
        ("1 + 2 + 3", plus(plus(1, 2), 3)),
        ("1 + 2 * 3", plus(1, times(2, 3))),
        ("1 * 2 + 3", plus(times(1, 2), 3)),
        ("1 - 2 - 3", minus(minus(1, 2), 3)),
        ("1 - 2 * 3", minus(1, times(2, 3))),
        ("1 * 2 - 3", minus(times(1, 2), 3)),
        ("1 * 2 * 3", times(times(1, 2), 3)),
        ("1 * !true", times(1, not(true))),
        ("!true * 1", times(not(true), 1)),
    ];
    assert_parses(cases, Parser::parse_expression);
}

#[test]
fn parse_expression_fail() {
    let cases = [
        "[",                // "[" is not a valid initial LHS.
        "(",                // No right parenthesis.
        "new",              // "new" on its own is not valid.
        "new int ",         // No brackets.
        "new int [",        // No right bracket.
        "new int []",       // Array length not specified.
        "new Foo",          // No parenthesis.
        "new Foo(",         // No right parenthesis.
        "array.",           // Nothing after the ".".
        "array.length(",    // No right parenthesis.
        "array.length(arg", // No right parenthesis.
        "array[",           // No right bracket.
        "1 + ",             // No RHS.
        "1 - ",             // No RHS.
        "1 * ",             // No RHS.
        "1 && ",            // No RHS.
        "1 < ",             // No RHS.
        " + 1",             // No LHS.
        " - 1",             // No LHS.
        " * 1",             // No LHS.
        " && 1",            // No LHS.
        " > 1",             // No LHS.
    ];
    assert_parses_fails(cases, Parser::parse_expression);
}
