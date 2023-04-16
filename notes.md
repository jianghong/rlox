[Rust implementations](https://github.com/munificent/craftinginterpreters/wiki/Lox-implementations#rust)

Grammar
```
program        → declaration* EOF ;
declaration    → varDecl
               | statement ;
varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
statement      → exprStmt
               | printStmt
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

expression     → equality ( ( "?" equality ":" equality )* | ( "," equality )* )* ;
ternary        → equality "?" equality ":" equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | "(" expression ")"
               | IDENTIFIER ;
```

# TODOs
- [x] Comparisons crash with stack overflow
- [x] https://craftinginterpreters.com/evaluating-expressions.html#runtime-errors
- [x] https://craftinginterpreters.com/evaluating-expressions.html extra credit section
- [ ] Implementing declarations and error recovery with syncronization https://craftinginterpreters.com/statements-and-state.html#parsing-variables