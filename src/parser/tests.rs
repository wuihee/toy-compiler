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

#[test]
fn precedence() {
    let cases = [
        ("1 + 2 * 3", plus(1, times(2, 3))),
        ("1 * 2 + 3", plus(times(1, 2), 3)),
    ];

    assert_parse(cases, Parser::parse_expression);
}
