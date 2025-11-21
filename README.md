# Toy Compiler

## Lexer

The lexer will target a tiny arithmetic language with variables.

### Token Types

- Literals: Decimals or floating-points.
- Identifiers: Start with `[A-za-z]` and can contain `[A-Za-z0-0_]`.
- Operators: `+`, `-`, `*`, `/`, `=`
- Delimiters: `(`, `)`
- End of Input

## Parser

The parser will parse the tiny langauge and construct the AST.
