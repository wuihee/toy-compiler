//! # Error Log
//!
//! When we encounter an error during name resolution or type checking, instead of panicking,
//! store the error in a log so we can display all errors to the user.

pub struct ErrorLog {}

impl ErrorLog {
    pub fn new() -> ErrorLog {
        ErrorLog {}
    }
}
