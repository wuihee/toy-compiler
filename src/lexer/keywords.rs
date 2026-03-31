//! # Keywords Lookup
//!
//! This module helps to match identifiers to keywords in the MiniJava
//! language.

use crate::lexer::token::{KeywordKind, TokenKind};

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
        "boolean" => Some(TokenKind::Keyword(KeywordKind::Boolean)),
        "class" => Some(TokenKind::Keyword(KeywordKind::Class)),
        "else" => Some(TokenKind::Keyword(KeywordKind::Else)),
        "extends" => Some(TokenKind::Keyword(KeywordKind::Extends)),
        "if" => Some(TokenKind::Keyword(KeywordKind::If)),
        "int" => Some(TokenKind::Keyword(KeywordKind::Int)),
        "main" => Some(TokenKind::Keyword(KeywordKind::Main)),
        "new" => Some(TokenKind::Keyword(KeywordKind::New)),
        "public" => Some(TokenKind::Keyword(KeywordKind::Public)),
        "return" => Some(TokenKind::Keyword(KeywordKind::Return)),
        "static" => Some(TokenKind::Keyword(KeywordKind::Static)),
        "System.out.println" => Some(TokenKind::Keyword(KeywordKind::SystemOutPrintln)),
        "this" => Some(TokenKind::Keyword(KeywordKind::This)),
        "void" => Some(TokenKind::Keyword(KeywordKind::Void)),
        "while" => Some(TokenKind::Keyword(KeywordKind::While)),
        "true" => Some(TokenKind::BooleanLiteral(true)),
        "false" => Some(TokenKind::BooleanLiteral(false)),
        _ => None,
    }
}
