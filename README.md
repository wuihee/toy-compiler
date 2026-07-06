# Toy Compiler

A toy compiler for the MiniJava language.

## Lexer

Run a demo of the lexer:

```sh
cargo run -- scan samples/Sample.java
```

## References

- Cooper, Keith D. & Torczon, Linda. _Engineering a Compiler_, 2nd ed. Morgan Kaufmann, 2011.
- Nystrom, Robert. _Crafting Interpreters_. Genever Benning, 2021.
- matklad. "Simple but Powerful Pratt Parsing." https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

### Grammar

```text
Goal
    ::= MainClass ( ClassDeclaration )* <EOF>

MainClass
    ::= "class" Identifier "{"
            "public" "static" "void" "main"
            "(" "String" "[" "]" Identifier ")"
            "{"
                Statement
            "}"
        "}"

ClassDeclaration
    ::= "class" Identifier ( "extends" Identifier )?
        "{"
            ( VarDeclaration )*
            ( MethodDeclaration )*
        "}"

VarDeclaration
    ::= Type Identifier ";"

MethodDeclaration
    ::= "public" Type Identifier
        "(" ( Type Identifier ( "," Type Identifier )* )? ")"
        "{"
            ( VarDeclaration )*
            ( Statement )*
            "return" Expression ";"
        "}"

Type
    ::= "int" "[" "]"
      | "boolean"
      | "int"
      | Identifier

Statement
    ::= "{"
            ( Statement )*
        "}"
      | "if" "(" Expression ")" Statement "else" Statement
      | "while" "(" Expression ")" Statement
      | "System.out.println" "(" Expression ")" ";"
      | Identifier "=" Expression ";"
      | Identifier "[" Expression "]" "=" Expression ";"

Expression
    ::= Expression ( "&&" | "<" | "+" | "-" | "*" ) Expression
      | Expression "[" Expression "]"
      | Expression "." "length"
      | Expression "." Identifier
            "(" ( Expression ( "," Expression )* )? ")"
      | <INTEGER_LITERAL>
      | "true"
      | "false"
      | Identifier
      | "this"
      | "new" "int" "[" Expression "]"
      | "new" Identifier "(" ")"
      | "!" Expression
      | "(" Expression ")"

Identifier
    ::= <IDENTIFIER>
```
