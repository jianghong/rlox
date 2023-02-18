mod rlox;
mod tool;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "generate_ast" {
        tool::generate_ast::main();
    } else if args.len() == 2 && args[1] == "ast_printer" {
        rlox::ast_printer::main();
    } else {
        let mut lox = rlox::lox::Lox::new();
        lox.main();
    }
}
