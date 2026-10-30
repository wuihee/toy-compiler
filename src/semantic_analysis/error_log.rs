//! # Error Log
//!
//! When we encounter an error during name resolution or type checking, instead of panicking,
//! store the error in a log so we can display all errors to the user.

use crate::span::Span;

/// The different error types that could occur during semantic analysis.
pub enum TypeError {
    /// An identifier was defined more than once.
    DuplicateIdentifier,
}

/// A log used to hold errors caught during semantic analysis.
pub struct ErrorLog {
    errors: Vec<(TypeError, Span)>,
}

impl ErrorLog {
    pub fn new() -> ErrorLog {
        ErrorLog { errors: Vec::new() }
    }

    /// Add a new error to the log.
    pub fn add_error(&mut self, error: TypeError, span: Span) {
        self.errors.push((error, span));
    }
}
