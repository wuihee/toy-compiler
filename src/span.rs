//! # Span

use std::fmt;

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

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.start, self.end)
    }
}
