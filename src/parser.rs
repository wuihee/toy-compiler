//! # Parser
//!
//! This module takes a stream of [`Token`]s, and transforms them into an AST.

mod errors;

use std::iter::Peekable;

use crate::{
    ast::{MainClass, Program},
    lexer::{Lexer, token::TokenKind},
    parser::errors::ParseError,
};

/// This struct transforms a stream of [`Token`]s into an AST.
pub struct Parser<'a> {
    /// A [`Lexer`] containing the stream of [`Token`]s from program.
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Instantiate a new [`Parser`].
    ///
    /// # Arguments
    ///
    /// - `lexer`: A [`Lexer`] holding the [`Token`]s from a MiniJava program.
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    /// Transforms a stream of [`Token`]s into an AST with a root of [`Program`].
    ///
    /// # Example
    ///
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        // Pre: lexer.peek() in FIRST(P).
        // Post: All tokens in P consumed.

        let main_class = self.parse_main_class()?;

        while let Some(token) = self.lexer.peek() {
            self.parse_class();
        }

        todo!()
    }

    fn parse_main_class(&mut self) -> Result<MainClass, ParseError> {
        // I don't know if this API is good design.
        // And to be honest, besides having a function per production, my mental model is non-existent.
        // Why is this even though I've spent so much time trying to apply my framework and model the
        // parser?
        self.parse_token(TokenKind::Main)?;

        todo!()
    }

    fn parse_class(&mut self) {}

    /// Checks that the next token is of `kind`.
    ///
    /// If it is, move to the next token. Otherwise, return an `Err`.
    fn parse_token(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if kind == self.lexer.peek().ok_or_else(|| ParseError::Temp)?.kind {
            self.lexer.next();

            return Ok(());
        }

        Err(ParseError::Temp)
    }
}
