//! # Parser Errors
//!
//!

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("TODO")]
    Temp,
}
