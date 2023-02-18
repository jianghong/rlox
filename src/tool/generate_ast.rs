use crate::rlox::token::Token;
use crate::rlox::token_type::TokenType;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

trait Expr {
    fn accept(&self);
}

struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>
}

impl Expr for Binary {
    fn accept(&self) {}
}

struct Literal {
    value: String
}

impl Expr for Literal {
    fn accept(&self) {}
}

pub fn main() {
    let mut ast: HashMap<String, Vec<(String, String)>> = HashMap::new();
    ast.insert(
        "Binary".to_string(),
        vec![
            ("left".to_string(), "Box<dyn Expr>".to_string()),
            ("operator".to_string(), "Token".to_string()),
            ("right".to_string(), "Box<dyn Expr>".to_string())
        ]
    );
    ast.insert(
        "Grouping".to_string(),
        vec![
            ("expression".to_string(), "Box<dyn Expr>".to_string())
        ]
    );
    ast.insert(
        "Literal".to_string(),
        vec![
            ("value".to_string(), "String".to_string())
        ]
    );
    ast.insert(
        "Unary".to_string(),
        vec![
            ("operator".to_string(), "Token".to_string()),
            ("right".to_string(), "Box<dyn Expr>".to_string())
        ]
    );
    let file = File::create("expr.rs").unwrap();
    let mut file = LineWriter::new(file);

    write_expr_trait(&mut file);
    write_ast(&mut file, &ast);

    file.write_all(b"\n").unwrap();
    file.flush().unwrap();
}

fn write_expr_trait(file: &mut LineWriter<File>) {
    file.write_all(b"trait Expr {\n").unwrap();
    file.write_all(b"    fn accept(&self);\n").unwrap();
    file.write_all(b"}\n\n").unwrap();
}

fn write_ast(file: &mut LineWriter<File>, ast: &HashMap<String, Vec<(String, String)>>) {
    for (name, fields) in ast {
        write_ast_class(file, name, fields);
    }
}

fn write_ast_class(file: &mut LineWriter<File>, name: &String, fields: &Vec<(String, String)>) {
    file.write_all(format!("struct {} {{\n", name).as_bytes()).unwrap();
    for (name, ty) in fields {
        file.write_all(format!("    {}: {},\n", name, ty).as_bytes()).unwrap();
    }
    file.write_all(b"}\n\n").unwrap();
    file.write_all(format!("impl Expr for {} {{\n", name).as_bytes()).unwrap();
    file.write_all(b"    fn accept(&self) {}\n").unwrap();
    file.write_all(b"}\n\n").unwrap();
}