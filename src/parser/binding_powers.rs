//! # Binding Powers for Operators
//!
//! This module contains methods to get the binding powers of operators, used for resolving
//! precedence in Pratt Parsing.

use crate::lexer::token::TokenKind;

/// !
pub fn prefix_binding_power(operator: TokenKind) -> Option<(u8, ())> {
    Some(match operator {
        TokenKind::Bang => (9, ()),
        _ => return None,
    })
}

/// &&, < , +, -, *
pub fn infix_binding_power(operator: TokenKind) -> Option<(u8, u8)> {
    Some(match operator {
        TokenKind::And => (1, 2),
        TokenKind::LessThan => (3, 4),
        TokenKind::Plus | TokenKind::Minus => (5, 6),
        TokenKind::Star => (7, 8),
        _ => return None,
    })
}

/// [, .
pub fn postfix_binding_power(operator: TokenKind) -> Option<((), u8)> {
    Some(match operator {
        TokenKind::Dot | TokenKind::LeftBracket => ((), 12),
        _ => return None,
    })
}
