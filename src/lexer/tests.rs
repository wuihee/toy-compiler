use super::*;

/// Test that `source` lexes into tokens of `kinds`.
fn test_lexer(source: &str, kinds: &[TokenKind]) {
    let mut lexer = Lexer::new(source);
    let mut i = 0;

    while let token = lexer.next_token()
        && token.kind != TokenKind::Eof
    {
        assert_eq!(token, Token::new(kinds[i].clone(), token.span.clone()));
        i += 1;
    }
}

/// Use to test a `source` that is a single lexeme.
fn test_lexeme(source: &str, kind: TokenKind) {
    test_lexer(source, &[kind]);
}

#[test]
fn eof() {
    let mut lexer = Lexer::new("");
    let token = lexer.next_token();

    assert_eq!(token, Token::new(TokenKind::Eof, Span::new(0, 0)))
}

#[test]
fn integer() {
    test_lexeme("42", TokenKind::IntegerLiteral(42));
    test_lexeme("007", TokenKind::IntegerLiteral(7));
}

#[test]
fn identifier() {
    test_lexeme("hello", TokenKind::Identifier(String::from("hello")));
    test_lexeme("hell0", TokenKind::Identifier(String::from("hell0")));
    test_lexeme("hell_o", TokenKind::Identifier(String::from("hell_o")));
}

#[test]
fn keywords() {
    test_lexeme("boolean", TokenKind::Boolean);
    test_lexeme("class", TokenKind::Class);
    test_lexeme("else", TokenKind::Else);
    test_lexeme("extends", TokenKind::Extends);
    test_lexeme("if", TokenKind::If);
    test_lexeme("int", TokenKind::Int);
    test_lexeme("length", TokenKind::Length);
    test_lexeme("main", TokenKind::Main);
    test_lexeme("new", TokenKind::New);
    test_lexeme("public", TokenKind::Public);
    test_lexeme("return", TokenKind::Return);
    test_lexeme("static", TokenKind::Static);
    test_lexeme("String", TokenKind::String);
    test_lexeme("this", TokenKind::This);
    test_lexeme("void", TokenKind::Void);
    test_lexeme("while", TokenKind::While);
}

#[test]
fn system_out_println() {
    test_lexeme("System.out.println", TokenKind::SystemOutPrintln);
}

#[test]
fn operators_and_delimiters() {
    test_lexeme("+", TokenKind::Plus);
    test_lexeme("-", TokenKind::Minus);
    test_lexeme("*", TokenKind::Star);
    test_lexeme("=", TokenKind::Equal);
    test_lexeme("&&", TokenKind::And);
    test_lexeme("<", TokenKind::LessThan);
    test_lexeme("!", TokenKind::Bang);
    test_lexeme("(", TokenKind::LeftParenthesis);
    test_lexeme(")", TokenKind::RightParenthesis);
    test_lexeme("[", TokenKind::LeftBracket);
    test_lexeme("]", TokenKind::RightBracket);
    test_lexeme("{", TokenKind::LeftBrace);
    test_lexeme("}", TokenKind::RightBrace);
    test_lexeme(",", TokenKind::Comma);
    test_lexeme(".", TokenKind::Dot);
    test_lexeme(";", TokenKind::Semicolon);
}

#[test]
fn unknown() {
    test_lexeme("%", TokenKind::Unknown('%'));
}

#[test]
fn simple() {
    let source = "int x = 1 + 2 * 3;";
    let kinds = vec![
        TokenKind::Int,
        TokenKind::Identifier(String::from("x")),
        TokenKind::Equal,
        TokenKind::IntegerLiteral(1),
        TokenKind::Plus,
        TokenKind::IntegerLiteral(2),
        TokenKind::Star,
        TokenKind::IntegerLiteral(3),
        TokenKind::Semicolon,
    ];

    test_lexer(source, &kinds);
}

#[test]
fn whitespace() {
    let source = "  \n \t\n   ";
    let mut lexer = Lexer::new(source);
    let token = lexer.next_token();
    let length = source.len();

    assert_eq!(token, Token::new(TokenKind::Eof, Span::new(length, length)));
}

#[test]
fn line_comment() {
    let source = "// this is a comment\nint x = 1 + 2 * 3;\n// Here's another comment\n// One more";
    let kinds = vec![
        TokenKind::Int,
        TokenKind::Identifier(String::from("x")),
        TokenKind::Equal,
        TokenKind::IntegerLiteral(1),
        TokenKind::Plus,
        TokenKind::IntegerLiteral(2),
        TokenKind::Star,
        TokenKind::IntegerLiteral(3),
        TokenKind::Semicolon,
    ];

    test_lexer(source, &kinds);
}

#[test]
fn block_comment() {
    let source = "/* This is a \n block comment \n\n***/ int x = 1 + 2 * 3; \n // Comment /**sdfa \n /* block */";
    let kinds = vec![
        TokenKind::Int,
        TokenKind::Identifier(String::from("x")),
        TokenKind::Equal,
        TokenKind::IntegerLiteral(1),
        TokenKind::Plus,
        TokenKind::IntegerLiteral(2),
        TokenKind::Star,
        TokenKind::IntegerLiteral(3),
        TokenKind::Semicolon,
    ];

    test_lexer(source, &kinds);
}

#[test]
fn block_comment_error() {
    let source = "/*";
    let kinds = vec![TokenKind::Unknown('/'), TokenKind::Star];

    test_lexer(source, &kinds);
}
