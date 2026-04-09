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
    Keyword(KeywordKind),
    Operator(OperatorKind),
    Delimiter(DelimeterKind),
    Eof,
    Unknown(char),
}

impl TokenKind {
    /// Matches a [`TokenKind`] to its corresponding lexeme.
    pub fn lexeme(&self) -> Option<&'static str> {
        match self {
            TokenKind::Operator(operator) => Some(operator.lexeme()),
            TokenKind::Delimiter(delimiter) => Some(delimiter.lexeme()),
            TokenKind::Keyword(keyword) => Some(keyword.lexeme()),
            _ => None,
        }
    }
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

impl KeywordKind {
    /// Matches a ['Keyword'] enum to its corresponding lexeme.
    fn lexeme(&self) -> &'static str {
        match self {
            KeywordKind::Boolean => "boolean",
            KeywordKind::Class => "class",
            KeywordKind::Else => "else",
            KeywordKind::Extends => "extends",
            KeywordKind::If => "if",
            KeywordKind::Int => "int",
            KeywordKind::Main => "main",
            KeywordKind::New => "new",
            KeywordKind::Public => "public",
            KeywordKind::Return => "return",
            KeywordKind::Static => "static",
            KeywordKind::SystemOutPrintln => "System.out.println",
            KeywordKind::This => "this",
            KeywordKind::Void => "void",
            KeywordKind::While => "while",
        }
    }
}

/// This enum represents an operator in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorKind {
    Add,
    Subtract,
    Multiply,
    Assign,
    And,
    LessThan,
    Not,
}

impl OperatorKind {
    /// Matches an [`Operator`] enum to its corresponding lexeme.
    fn lexeme(&self) -> &'static str {
        match self {
            OperatorKind::Add => "+",
            OperatorKind::Subtract => "-",
            OperatorKind::Multiply => "*",
            OperatorKind::Assign => "=",
            OperatorKind::And => "&&",
            OperatorKind::LessThan => "<",
            OperatorKind::Not => "!",
        }
    }
}

/// This enum represents a delimiter in the MiniJava language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimeterKind {
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

impl DelimeterKind {
    /// Matches a [`Delimiter`] enum to its corresponding lexeme.
    fn lexeme(&self) -> &'static str {
        match self {
            DelimeterKind::LeftParenthesis => "(",
            DelimeterKind::RightParenthesis => ")",
            DelimeterKind::LeftBracket => "[",
            DelimeterKind::RightBracket => "]",
            DelimeterKind::LeftBrace => "{",
            DelimeterKind::RightBrace => "}",
            DelimeterKind::Comma => ",",
            DelimeterKind::Dot => ".",
            DelimeterKind::Semicolon => ";",
        }
    }
}
