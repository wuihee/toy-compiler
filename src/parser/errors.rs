//! # Parser Errors
//!
//! This module holds all parser-related errors.

use thiserror::Error;

use crate::{
    lexer::token::TokenKind,
    span::{LineIndex, Span},
};

/// Contains the errors encountered during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Got nothing when a token was expected.
    #[error("Unexpected EOF encountered.")]
    UnexpectedEof,

    /// An unexpected token was received.
    #[error("Unexpected token '{kind}'.")]
    UnexpectedToken { kind: TokenKind, span: Span },
}

pub fn format_error(source: &str, error: &ParseError) -> String {
    match error {
        ParseError::UnexpectedEof => error.to_string(),
        ParseError::UnexpectedToken { kind, span } => {
            let line_index = LineIndex::new(source);
            let offset = span.start;
            let location = line_index.location(offset);
            let error_line = source.lines().nth(location.line).unwrap_or("");

            format!(
                "Unexpected token '{}' at line {} column {}.\n|\n| {}\n|{}^",
                kind,
                location.line + 1,
                location.column + 1,
                error_line,
                " ".repeat(location.column + 1)
            )
        }
    }
}
