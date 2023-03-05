[Rust implementations](https://github.com/munificent/craftinginterpreters/wiki/Lox-implementations#rust)

Grammar
```
expression     → equality ( ( "?" equality ":" equality )* | ( "," equality )* )* ;
ternary        → equality "?" equality ":" equality
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```

# TODOs
- [x] Comparisons crash with stack overflow
- [x] https://craftinginterpreters.com/evaluating-expressions.html#runtime-errors
- [x] https://craftinginterpreters.com/evaluating-expressions.html extra credit section
