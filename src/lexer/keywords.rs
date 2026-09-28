//! # Keywords Lookup
//!
//! This module helps to match identifiers to keywords in the MiniJava
//! language.

use crate::lexer::token::TokenKind;

/// Matches a string to a keyword.
///
/// # Arguments
///
/// - `identifier` - The string to check.
///
/// # Example
///
/// ```rust
/// use toy_compiler::lexer::{keywords, token::TokenKind};
///
/// let keyword = keywords::lookup_keyword("boolean");
///
/// assert_eq!(keyword, Some(TokenKind::Boolean));
/// ```
pub fn lookup_keyword(identifier: &str) -> Option<TokenKind> {
    let kind = match identifier {
        "boolean" => TokenKind::Boolean,
        "class" => TokenKind::Class,
        "else" => TokenKind::Else,
        "extends" => TokenKind::Extends,
        "if" => TokenKind::If,
        "int" => TokenKind::Int,
        "main" => TokenKind::Main,
        "new" => TokenKind::New,
        "public" => TokenKind::Public,
        "return" => TokenKind::Return,
        "static" => TokenKind::Static,
        "String" => TokenKind::String,
        "System.out.println" => TokenKind::SystemOutPrintln,
        "this" => TokenKind::This,
        "void" => TokenKind::Void,
        "while" => TokenKind::While,
        "true" => TokenKind::BooleanLiteral(true),
        "false" => TokenKind::BooleanLiteral(false),
        _ => return None,
    };

    Some(kind)
}
