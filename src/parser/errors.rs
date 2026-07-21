//! # Parser Errors
//!
//! This module holds all parser-related errors.

use thiserror::Error;

use crate::{lexer::token::TokenKind, span::Span};

/// Contains the errors encountered during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Got nothing when a token was expected.
    #[error("Unexpected EOF encountered.")]
    UnexpectedEof,

    /// An unexpected token was received.
    #[error("Unexpected token '{kind}' at {span}.")]
    UnexpectedToken { kind: TokenKind, span: Span },
}
