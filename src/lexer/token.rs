//! # Token Definitions for MiniJava
//!
//! This module defines the lexical vocabulary of MiniJava.

/// A single lexeme produced by the lexer.
#[derive(Debug)]
pub struct Token {
    /// The lexical category of this token.
    pub kind: TokenKind,

    /// The location of this token in the source text.
    pub span: Span,
}

/// A byte range within the source text.
///
/// `Span` uses `[start, end)` indexing.
#[derive(Debug)]
pub struct Span {
    /// The starting byte offset (inclusive).
    pub start: usize,

    /// The ending byte offset (exclusive).
    pub end: usize,
}

/// The lexical category of a token.
#[derive(Debug)]
pub enum TokenKind {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
}

/// This enum represents a keyword in the MiniJava language.
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Assign,
    And,
    LessThan,
    Not,
}

/// This enum represents a delimiter in the MiniJava language.
#[derive(Debug)]
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
