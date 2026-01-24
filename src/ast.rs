//! # AST
//!
//! This module contains the data structures that make up the AST.

/// Currently, an entire program consists of a list of statements.
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A statement is a line of code.
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment { name: String, value: Expression },
    Expression { value: Expression },
}

/// An `Expression` are the "units" of our language.
#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Integer(i64),
    Identifier(String),
}

/// Possible operators we can use in our language.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
