# Toy Compiler

## Lexer

The lexer will target a tiny arithmetic language with variables.

### Token Types

- Literals: Decimals or floating-points.
- Identifiers: Start with `[A-za-z]` and can contain `[A-Za-z0-0_]`.
- Operators: `+`, `-`, `*`, `/`, `=`
- Delimiters: `(`, `)`
- End of Input

### Lexer Demo

To the run the scanner use the `scan` command followed by the file to scan.

```sh
cargo run -- scan sample.txt
```

This is the samle output which shows the tokens from `sample.txt`.

```text
Identifier("x") Operator(Equals) Literal("1") Operator(Plus) Literal("2") Operator(Multiply) Delimiter(LeftParenthesis) Literal("3") Operator(Divide) Literal("4") Operator(Multiply) Literal("3") Delimiter(RightParenthesis) Operator(Minus) Literal("5") Identifier("y") Operator(Equals) Identifier("x") Operator(Multiply) Literal("2") Eof
```

## Parser

The parser will parse the tiny langauge and construct the AST.

```math
\begin{align*}
  \text{Program} &::= \text{Statement}^* \text{ EOF} \\
  \text{Statement} &::= \text{IDENTIFIER} = \text{Expression}; \mid \text{Expression} ; \\
  \text{Expression} &::= \text{Term } ((+ \mid -) \text{ Term})^* \mid \text{Term}\\
  \text{Term} &::= \text{Factor } ((* \mid /) \text{ Factor})^* \mid \text{Factor} \\
  \text{Factor} &::= \text{NUMBER} \mid \text{IDENTIFIER} \mid (\text{Expression})
\end{align*}
```

The `parse` command will tokenize the given file and construct and print an AST.

```sh
cargo run -- parse sample.txt
```
