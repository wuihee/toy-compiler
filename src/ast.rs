//! # AST
//!
//! This module contains the data structures that make up the AST.

/// Currently, an entire program consists of a list of statements.
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A statement is a line of code. Currently we only have assignment.
pub enum Statement {
    Assignment { name: Identifier, value: Expression },
}

/// For now, an identifier is just a `String`.
pub type Identifier = String;

/// An `Expression` are the "units" of our language.
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Number(i64),
    Variable(Identifier),
}

/// Possible operators we can use in our language.
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
