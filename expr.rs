trait Expr {
    fn accept(&self);
}

struct Literal {
    value: String,
}

impl Expr for Literal {
    fn accept(&self) {}
}

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self) {}
}

struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self) {}
}

struct Grouping {
    expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self) {}
}


