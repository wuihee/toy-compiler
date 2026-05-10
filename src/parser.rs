//! # Parser
//!
//! This module takes a stream of [`Token`]s, and transforms them into an AST.

use std::iter::Peekable;

use thiserror::Error;

use crate::{
    ast::Program,
    lexer::{
        Lexer,
        token::{Token, TokenKind},
    },
};

#[derive(Debug, Error)]
pub enum ParseError {}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> Program {
        match self.lexer.peek() {
            Some(token) => match &token.kind {
                TokenKind::Identifier(s) if s == "class" => self.main_class(),
                _ => todo!("Throw error"),
            },
            None => {}
        }

        todo!()
    }

    fn main_class(&mut self) {}
}
