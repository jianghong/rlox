use super::{
    error_reporter::ErrorReporter, interpreter::Interpreter, parser::Parser, scanner::Scanner,
};

pub struct Lox {
    error_reporter: ErrorReporter,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            error_reporter: ErrorReporter::new(),
            interpreter: Interpreter::new(),
        }
    }

    pub fn main(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 2 {
            panic!("Usage: rlox [script]");
        } else if args.len() == 2 {
            self.run_file(args[1].clone());
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, script: String) {
        let source = std::fs::read_to_string(script).unwrap();
        self.run(source)
    }

    fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.run(input);
            self.error_reporter.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, &mut self.error_reporter);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens, &mut self.error_reporter);

        if let Ok(expr) = parser.parse() {
            // let mut ast_printer = AstPrinter::new();
            // let ast = expr.accept(&mut ast_printer);
            // println!("{}", ast)
            self.interpreter.interpret(&expr, &mut self.error_reporter)
        }
    }
}
