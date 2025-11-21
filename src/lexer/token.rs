//! # Token
//!
//! Contains the structs representing tokens in the tiny language.

/// Represents a single token in the tiny language.
pub enum Token {
    Literal(String),
    Identifier(String),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
}

/// Represents the different operator token types.
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
}

/// Represents the different delimiter token types.
pub enum Delimiter {
    LeftParenthesis,
    RightParenthesis,
}
