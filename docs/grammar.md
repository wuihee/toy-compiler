# MiniJava Grammar

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

## Expression precedence

Loosest to tightest, following [Java's precedence rules](https://introcs.cs.princeton.edu/java/11precedence/):

| Operator | Binding power | Associativity |
| -------- | ------------- | ------------- |
| `&&`     | 1, 2          | left          |
| `<`      | 3, 4          | left          |
| `+` `-`  | 5, 6          | left          |
| `*`      | 7, 8          | left          |
| `!`      | 9 (prefix)    | right         |
| `.` `[`  | 12 (postfix)  | left          |
