# Toy Compiler

[![Rust](https://github.com/wuihee/toy-compiler/actions/workflows/rust.yml/badge.svg)](https://github.com/wuihee/toy-compiler/actions/workflows/rust.yml)

A compiler for [MiniJava](docs/grammar.md), written from scratch in Rust.

## Status

| Stage                   | State       |
| ----------------------- | ----------- |
| Lexer                   | Done        |
| Parser                  | In progress |
| Type Checking           | Planned     |
| Interpreter             | Planned     |
| IR + Control Flow Graph | Planned     |
| Optimization            | Planned     |
| Code Generation         | Planned     |

## Usage

Each subcommand runs the pipeline up to one stage and prints what that stage
produced, which is how you watch the compiler work on a program of your own.

| Command        | Output                               |
| -------------- | ------------------------------------ |
| `scan <file>`  | The token stream, one token per line |
| `parse <file>` | The AST, pretty-printed              |

```sh
cargo run -- scan samples/Sample.java
cargo run -- parse samples/Sample.java
```

## Docs

- [Grammar](docs/grammar.md): the MiniJava grammar and expression precedence.

## References

- Cooper, Keith D. & Torczon, Linda. _Engineering a Compiler_, 2nd ed. Morgan Kaufmann, 2011.
- Nystrom, Robert. _Crafting Interpreters_. Genever Benning, 2021.
- Rust Compiler Source. rust-lang/rust. https://github.com/rust-lang/rust
- matklad. "Simple but Powerful Pratt Parsing." https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
