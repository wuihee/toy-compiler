//! # Token Definitions for MiniJava
//!
//! This module defines the lexical vocabulary of MiniJava.

/// A single lexeme produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The lexical category of this token.
    pub kind: TokenKind,

    /// The location of this token in the source text.
    pub span: Span,
}

/// A byte range within the source text.
///
/// `Span` uses `[start, end)` indexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// The starting byte offset (inclusive).
    pub start: usize,

    /// The ending byte offset (exclusive).
    pub end: usize,
}

/// The lexical category of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
}

impl TokenKind {
    /// Matches a [`TokenKind`] to its corresponding lexeme.
    pub fn lexeme(&self) -> &'static str {
        match self {
            TokenKind::Operator(operator) => operator.lexeme(),
            TokenKind::Delimiter(delimiter) => delimiter.lexeme(),
            _ => todo!(),
        }
    }
}

/// This enum represents a keyword in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Boolean,
    Class,
    Else,
    Extends,
    If,
    Int,
    Main,
    New,
    Public,
    Return,
    Static,
    SystemOutPrintln,
    This,
    Void,
    While,
}

/// This enum represents an operator in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Assign,
    And,
    LessThan,
    Not,
}

impl Operator {
    /// Matches an [`Operator`] enum to its corresponding lexeme.
    fn lexeme(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Assign => "=",
            Operator::And => "&&",
            Operator::LessThan => "<",
            Operator::Not => "!",
        }
    }
}

/// This enum represents a delimiter in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
}

impl Delimiter {
    /// Matches an [`Delimiter`] enum to its corresponding lexeme.
    fn lexeme(&self) -> &'static str {
        match self {
            Delimiter::LeftParenthesis => "(",
            Delimiter::RightParenthesis => ")",
            Delimiter::LeftBracket => "[",
            Delimiter::RightBracket => "]",
            Delimiter::LeftBrace => "{",
            Delimiter::RightBrace => "}",
            Delimiter::Comma => ",",
            Delimiter::Dot => ",",
            Delimiter::Semicolon => ";",
        }
    }
}
