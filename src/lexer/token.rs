//! # Token Definitions for MiniJava
//!
//! This module defines the lexical vocabulary of MiniJava.

use std::fmt::Display;

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
    Boolean,
    String,
    Class,
    Else,
    Extends,
    If,
    Int,
    Length,
    Main,
    New,
    Public,
    Return,
    Static,
    SystemOutPrintln,
    This,
    Void,
    While,
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
    Unknown(char),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::IntegerLiteral(val) => write!(f, "{val}"),
            TokenKind::BooleanLiteral(val) => write!(f, "{val}"),
            TokenKind::Identifier(val) => write!(f, "{val}"),
            TokenKind::Boolean => write!(f, "boolean"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Extends => write!(f, "extends"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Int => write!(f, "int"),
            TokenKind::Length => write!(f, "length"),
            TokenKind::Main => write!(f, "main"),
            TokenKind::New => write!(f, "new"),
            TokenKind::Public => write!(f, "public"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Static => write!(f, "static"),
            TokenKind::SystemOutPrintln => write!(f, "System.out.println"),
            TokenKind::This => write!(f, "this"),
            TokenKind::Void => write!(f, "void"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::And => write!(f, "&&"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::LeftParenthesis => write!(f, "("),
            TokenKind::RightParenthesis => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Unknown(val) => write!(f, "{val}"),
        }
    }
}
