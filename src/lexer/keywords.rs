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
/// ```rs
/// use lexer::keywords;
///
/// let keyword = keywords::lookup_keyword("boolean");
///
/// assert_eq!(keyword, Some(TokenKind::Keyword(KeywordKind::Boolean)));
/// ```
pub fn lookup_keyword(identifier: &str) -> Option<TokenKind> {
    match identifier {
        "boolean" => Some(TokenKind::Boolean),
        "class" => Some(TokenKind::Class),
        "else" => Some(TokenKind::Else),
        "extends" => Some(TokenKind::Extends),
        "if" => Some(TokenKind::If),
        "int" => Some(TokenKind::Int),
        "main" => Some(TokenKind::Main),
        "new" => Some(TokenKind::New),
        "public" => Some(TokenKind::Public),
        "return" => Some(TokenKind::Return),
        "static" => Some(TokenKind::Static),
        "System.out.println" => Some(TokenKind::SystemOutPrintln),
        "this" => Some(TokenKind::This),
        "void" => Some(TokenKind::Void),
        "while" => Some(TokenKind::While),
        "true" => Some(TokenKind::BooleanLiteral(true)),
        "false" => Some(TokenKind::BooleanLiteral(false)),
        _ => None,
    }
}
