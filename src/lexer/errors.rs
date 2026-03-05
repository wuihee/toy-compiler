//! # Lexer Errors
//!
//! This module defines error types for the lexer.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected token")]
    UnexpectedToken,
}
