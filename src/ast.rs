//! # AST
//!
//! This module contains the data structures that make up the AST.

use crate::lexer::token::Operator;

/// Currently, an entire program consists of a list of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A statement is a line of code.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment { name: String, value: Expression },
    Expression { value: Expression },
}

/// An `Expression` are the "units" of our language.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<&Operator> for BinaryOperator {
    type Error = ();

    fn try_from(value: &Operator) -> Result<Self, Self::Error> {
        Ok(match value {
            Operator::Plus => BinaryOperator::Add,
            Operator::Minus => BinaryOperator::Subtract,
            Operator::Multiply => BinaryOperator::Multiply,
            Operator::Divide => BinaryOperator::Divide,
            _ => return Err(()),
        })
    }
}
