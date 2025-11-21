//! # Token
//!
//! Contains the structs representing tokens in the tiny language.

/// Represents a single token in the tiny language.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Literal(String),
    Identifier(String),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
}

/// Represents the different operator token types.
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
}

/// Represents the different delimiter token types.
#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    LeftParenthesis,
    RightParenthesis,
}
