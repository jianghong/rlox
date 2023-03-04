#[cfg(test)]
pub mod tests {
    use crate::rlox::scanner::Scanner;
    use crate::rlox::parser::Parser;
    use crate::rlox::error_reporter::ErrorReporter;
    use crate::rlox::expr::Expr;

    pub fn helper_create_expr_from_string(expression: &str) -> Expr {
        let mut error_reporter = ErrorReporter::new();
        let mut scanner = Scanner::new(
            expression.to_string(),
            &mut error_reporter
        );
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens, &mut error_reporter);
        parser.parse().unwrap()
    }
}