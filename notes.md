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
- [ ] Comparisons crash with stack overflow
- [ ] https://craftinginterpreters.com/evaluating-expressions.html#runtime-errors
