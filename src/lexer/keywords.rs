//! # Keywords Lookup
//!
//! This module helps to match identifiers to keywords in the MiniJava
//! language.

use crate::lexer::token::{Keyword, TokenKind};

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
/// assert_eq!(keyword, Some(TokenKind::Keyword(Keyword::Boolean)));
/// ```
pub fn lookup_keyword(identifier: &str) -> Option<TokenKind> {
    match identifier {
        "boolean" => Some(TokenKind::Keyword(Keyword::Boolean)),
        "class" => Some(TokenKind::Keyword(Keyword::Class)),
        "else" => Some(TokenKind::Keyword(Keyword::Else)),
        "extends" => Some(TokenKind::Keyword(Keyword::Extends)),
        "if" => Some(TokenKind::Keyword(Keyword::If)),
        "int" => Some(TokenKind::Keyword(Keyword::Int)),
        "main" => Some(TokenKind::Keyword(Keyword::Main)),
        "new" => Some(TokenKind::Keyword(Keyword::New)),
        "public" => Some(TokenKind::Keyword(Keyword::Public)),
        "return" => Some(TokenKind::Keyword(Keyword::Return)),
        "static" => Some(TokenKind::Keyword(Keyword::Static)),
        "System.out.println" => Some(TokenKind::Keyword(Keyword::SystemOutPrintln)),
        "this" => Some(TokenKind::Keyword(Keyword::This)),
        "void" => Some(TokenKind::Keyword(Keyword::Void)),
        "while" => Some(TokenKind::Keyword(Keyword::While)),
        "true" => Some(TokenKind::BooleanLiteral(true)),
        "false" => Some(TokenKind::BooleanLiteral(false)),
        _ => None,
    }
}
