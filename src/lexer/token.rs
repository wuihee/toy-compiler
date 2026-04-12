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

impl Token {
    /// Helper function to create a new [`Token`].
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
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

impl Span {
    /// Helper function to create a new [`Span`].
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}

/// The lexical category of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    Identifier(String),
    Keyword(KeywordKind),
    Plus,
    Minus,
    Star,
    Equal,
    And,
    LessThan,
    Bang,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Eof,
    Unknown(char),
}

/// This enum represents a keyword in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordKind {
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
