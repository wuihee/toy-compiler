# Expression Precedence

Loosest to tightest, following [Java's precedence rules](https://introcs.cs.princeton.edu/java/11precedence/):

| Operator | Binding power | Associativity |
| -------- | ------------- | ------------- |
| `&&`     | 1, 2          | left          |
| `<`      | 3, 4          | left          |
| `+` `-`  | 5, 6          | left          |
| `*`      | 7, 8          | left          |
| `!`      | 9 (prefix)    | right         |
| `.` `[`  | 12 (postfix)  | left          |
