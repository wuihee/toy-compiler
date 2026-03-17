//! # Lexer Errors
//!
//! This module defines error types for the lexer.

use thiserror::Error;

use crate::lexer::token::Span;

/// This type represents all possible errors when lexing a program.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LexerError {
    /// Lexer encountered a symbol not recognized in the MiniJava language.
    #[error("Unexpected symbol '{symbol}' at {span:?}")]
    UnexpectedSymbol { symbol: char, span: Span },
}
