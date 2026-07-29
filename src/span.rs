//! # Span
//!
//! This module is the layer of abstraction providing the structs that deal with the identifying
//! locations of tokens in source programs.

use std::iter;

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

/// A location pointing to a line number and column offset within some input program
/// provided to the compiler.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// The line number of the location.
    pub line: usize,

    /// Column offset of the location.
    pub column: usize,
}

/// This struct helps with calculating the [`Location`] of in a source program.
pub struct LineIndex {
    /// The byte position of each new line in `source`.
    line_starts: Vec<usize>,
}

impl LineIndex {
    /// Initialize a new [`LineIndex`] from a `source` program that we're trying to compile.
    pub fn new(source: &str) -> LineIndex {
        let line_starts: Vec<usize> = iter::once(0)
            .chain(source.match_indices('\n').map(|(index, _)| index + 1))
            .collect();

        LineIndex { line_starts }
    }

    /// Map a byte offset in `source` to a [`Location`].
    pub fn location(&self, offset: usize) -> Location {
        let line = self
            .line_starts
            .partition_point(|&line_start| line_start <= offset)
            - 1;
        let column = offset - self.line_starts[line];

        Location { line, column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location() {
        let source = r#"This is line 0.
            This is line 1.
            Hello, World!"#;
        let line_index = LineIndex::new(source);

        assert_eq!(line_index.location(0), Location { line: 0, column: 0 });
        assert_eq!(line_index.location(18), Location { line: 1, column: 2 });
    }
}
