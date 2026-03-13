//! # Lexer Errors
//!
//! This module defines error types for the lexer.

use thiserror::Error;

/// This type represents all possible errors when lexing a program.
#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected token")]
    UnexpectedToken,
}
