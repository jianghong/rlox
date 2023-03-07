use rlox::core;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "ast_printer" {
        core::ast_printer::main();
    } else {
        let mut lox = core::lox::Lox::new();
        lox.main();
    }
}
