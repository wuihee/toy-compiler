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

fn class(
    name: &str,
    super_class: Option<&str>,
    fields: impl Into<Vec<Variable>>,
    methods: impl Into<Vec<Method>>,
) -> Class {
    Class {
        name: Identifier::new(name),
        super_class: super_class.map(Identifier::new),
        fields: fields.into(),
        methods: methods.into(),
    }
}

fn method(
    return_type: Type,
    name: &str,
    parameters: impl Into<Vec<Variable>>,
    variables: impl Into<Vec<Variable>>,
    body: impl Into<Vec<Statement>>,
    return_expression: impl Into<Expression>,
) -> Method {
    Method {
        return_type,
        name: Identifier::new(name),
        parameters: parameters.into(),
        variables: variables.into(),
        body: body.into(),
        return_expression: return_expression.into(),
    }
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

macro_rules! block {
    ($($statement:expr),* $(,)?) => {
        Statement::Block {
            statements: vec![$($statement),*]
        }
    };
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

fn if_else(
    condition: impl Into<Expression>,
    if_branch: impl Into<Statement>,
    else_branch: impl Into<Statement>,
) -> Statement {
    Statement::If {
        condition: condition.into(),
        then_branch: Box::new(if_branch.into()),
        else_branch: Box::new(else_branch.into()),
    }
}

fn while_loop(value: impl Into<Expression>, statement: impl Into<Statement>) -> Statement {
    Statement::While {
        condition: value.into(),
        body: Box::new(statement.into()),
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

fn array_assign(
    array: &str,
    index: impl Into<Expression>,
    value: impl Into<Expression>,
) -> Statement {
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
        "[",
        "(",
        "new",
        "new int ",
        "new int [",
        "new int []",
        "new Foo",
        "new Foo(",
        "array.",
        "array.length(",
        "array.length(arg",
        "array[",
        "1 + ",
        "1 - ",
        "1 * ",
        "1 && ",
        "1 < ",
        " + 1",
        " - 1",
        " * 1",
        " && 1",
        " > 1",
    ];
    assert_parses_fails(cases, Parser::parse_expression);
}

#[test]
fn parse_statement() {
    let cases = [
        ("{}", block!()),
        (
            indoc! {"
                {
                    System.out.println(1);
                }
            "},
            block!(println(1)),
        ),
        (
            indoc! {"
                {
                    System.out.println(1);
                    foo = 1;
                }
            "},
            block!(println(1), assign("foo", 1)),
        ),
        (
            "if (true) System.out.println(1); else System.out.println(2);",
            if_else(true, println(1), println(2)),
        ),
        (
            indoc! {"
                if (1 + 1 < 0) {
                    System.out.println(1 + 1);
                } else {
                    array[1 + 1] = foo;
                }
            "},
            if_else(
                less_than(plus(1, 1), 0),
                block!(println(plus(1, 1))),
                block!(array_assign("array", plus(1, 1), "foo")),
            ),
        ),
        (
            "while (true) System.out.println(1);",
            while_loop(true, println(1)),
        ),
        (
            indoc! {"
                while (i < 10) {
                    System.out.println(foo);

                    while (j < 5) {
                        array[0] = array[0] + 1;
                    }
                }
            "},
            while_loop(
                less_than("i", 10),
                block!(
                    println("foo"),
                    while_loop(
                        less_than("j", 5),
                        block!(array_assign("array", 0, plus(array_lookup("array", 0), 1)))
                    )
                ),
            ),
        ),
        ("System.out.println(1);", println(1)),
        ("foo = 1;", assign("foo", 1)),
        ("array[0] = 1;", array_assign("array", 0, 1)),
    ];
    assert_parses(cases, Parser::parse_statement);
}

#[test]
fn parse_statement_fail() {
    let cases = [
        "{",
        "}",
        "if",
        "if )",
        "if ({}) {} else {}",
        "if ( {} else {}",
        "if (true) true else {}",
        "if (true) {} else false",
        "while",
        "while (",
        "while true",
        "while (true",
        "while (true) 1",
        "System.out.println;",
        "System.out.println(;",
        "System.out.println({};",
        "System.out.println({});",
        "System.out.println(1)",
        "array = {};",
        "array = 1",
        "array] = 0;",
        "array[ = 0;",
        "array[{}] = 0;",
        "array[0] = 0",
    ];
    assert_parses_fails(cases, Parser::parse_statement);
}

#[test]
fn parse_method() {
    let cases = [(
        indoc! {"
            public int foo(int x, boolean y) {
                int a;
                int[] b;

                b = 0;

                if (y) {
                    a = a + x;
                } else {
                    a = b + x;
                }

                return a;
            }
        "},
        method(
            Type::Integer,
            "foo",
            vec![variable(Type::Integer, "x"), variable(Type::Boolean, "y")],
            vec![
                variable(Type::Integer, "a"),
                variable(Type::IntegerArray, "b"),
            ],
            vec![
                assign("b", 0),
                if_else(
                    "y",
                    block!(assign("a", plus("a", "x"))),
                    block!(assign("a", plus("b", "x"))),
                ),
            ],
            "a",
        ),
    )];
    assert_parses(cases, Parser::parse_method);
}

#[test]
fn parse_class() {
    let cases = [(
        indoc! {"
            class Foo extends Bar {
                int a;
                boolean b;

                public int spam() {
                    return 1;
                }

                public boolean eggs() {
                    return true;
                }
            }
        "},
        class(
            "Foo",
            Some("Bar"),
            vec![variable(Type::Integer, "a"), variable(Type::Boolean, "b")],
            vec![
                method(Type::Integer, "spam", vec![], vec![], vec![], int(1)),
                method(Type::Boolean, "eggs", vec![], vec![], vec![], boolean(true)),
            ],
        ),
    )];
    assert_parses(cases, Parser::parse_class);
}

#[test]
fn parse_main_class() {
    let cases = [(
        indoc! {"
            class Main {
                public static void main(String[] args) {
                    System.out.println(1);
                }
            }
        "},
        MainClass {
            name: Identifier::new("Main"),
            body: println(int(1)),
        },
    )];
    assert_parses(cases, Parser::parse_main_class);
}

#[test]
fn parse() {
    let cases = [(
        indoc! {"
                class Main {
                    public static void main(String[] args) {
                        System.out.println(1);
                    }
                }

                class Bar {}

                class Foo extends Bar {
                    int a;
                    boolean b;

                    public int spam() {
                        return 1;
                    }

                    public boolean eggs() {
                        return true;
                    }
                }
            "},
        Program {
            main: MainClass {
                name: Identifier::new("Main"),
                body: println(int(1)),
            },
            classes: vec![
                class("Bar", None, vec![], vec![]),
                class(
                    "Foo",
                    Some("Bar"),
                    vec![variable(Type::Integer, "a"), variable(Type::Boolean, "b")],
                    vec![
                        method(Type::Integer, "spam", vec![], vec![], vec![], int(1)),
                        method(Type::Boolean, "eggs", vec![], vec![], vec![], boolean(true)),
                    ],
                ),
            ],
        },
    )];
    assert_parses(cases, Parser::parse);
}

#[test]
fn parse_fail() {
    let cases = [indoc! {"
        class Main {
            public static void main(String[] args) {
                System.out.println(1);
            }
        }

        class Bar {}

        sdf
    "}];
    assert_parses_fails(cases, Parser::parse);
}
