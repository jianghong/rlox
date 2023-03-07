#[cfg(test)]
pub mod tests {
    use crate::core::{
        error_reporter::ErrorReporter, expr::Expr, parser::Parser, scanner::Scanner,
    };

    pub fn helper_create_expr_from_string(expression: &str) -> Expr {
        let mut error_reporter = ErrorReporter::new();
        let mut scanner = Scanner::new(expression.to_string(), &mut error_reporter);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens, &mut error_reporter);
        parser.parse().unwrap()
    }
}
