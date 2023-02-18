use crate::rlox::token::Token;
use crate::rlox::token_type::TokenType;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary<T>) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
}

trait Expr<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

struct Binary<T> {
    left: Box<dyn Expr<T>>,
    operator: Token,
    right: Box<dyn Expr<T>>
}

impl<T> Expr<T> for Binary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

struct Literal {
    value: String
}

impl<T> Expr<T> for Literal {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

pub fn main() {
    let mut ast: HashMap<String, Vec<(String, String)>> = HashMap::new();
    ast.insert(
        "Binary<T>".to_string(),
        vec![
            ("left".to_string(), "Box<dyn Expr<T>>".to_string()),
            ("operator".to_string(), "Token".to_string()),
            ("right".to_string(), "Box<dyn Expr<T>>".to_string())
        ]
    );
    ast.insert(
        "Grouping<T>".to_string(),
        vec![
            ("expression".to_string(), "Box<dyn Expr<T>>".to_string())
        ]
    );
    ast.insert(
        "Literal".to_string(),
        vec![
            ("value".to_string(), "Option<String>".to_string())
        ]
    );
    ast.insert(
        "Unary<T>".to_string(),
        vec![
            ("operator".to_string(), "Token".to_string()),
            ("right".to_string(), "Box<dyn Expr<T>>".to_string())
        ]
    );
    let file = File::create("src/rlox/expr.rs").unwrap();
    let mut file = LineWriter::new(file);
    
    write_imports(&mut file);
    write_expr_trait(&mut file);
    write_ast(&mut file, &ast);
    write_visitor_trait(&mut file, &ast);

    file.write_all(b"\n").unwrap();
    file.flush().unwrap();
}

fn write_imports(file: &mut LineWriter<File>) {
    file.write_all(b"use crate::rlox::token::Token;\n\n").unwrap();
}

fn write_expr_trait(file: &mut LineWriter<File>) {
    file.write_all(b"pub trait Expr<T> {\n").unwrap();
    file.write_all(b"    fn accept(&self, visitor: &dyn Visitor<T>) -> T;\n").unwrap();
    file.write_all(b"}\n\n").unwrap();
}

fn write_ast(file: &mut LineWriter<File>, ast: &HashMap<String, Vec<(String, String)>>) {
    for (name, fields) in ast {
        write_ast_class(file, name, fields);
    }
}

fn write_ast_class(file: &mut LineWriter<File>, name: &String, fields: &Vec<(String, String)>) {
    file.write_all(format!("pub struct {} {{\n", name).as_bytes()).unwrap();
    for (name, ty) in fields {
        file.write_all(format!("    pub {}: {},\n", name, ty).as_bytes()).unwrap();
    }
    let raw_name = name.split('<').next().unwrap();
    file.write_all(b"}\n\n").unwrap();
    file.write_all(format!("impl<T> Expr<T> for {} {{\n", name).as_bytes()).unwrap();
    file.write_all(format!("    fn accept(&self, visitor: &dyn Visitor<T>) -> T {{\n").as_bytes()).unwrap();
    file.write_all(format!("        visitor.visit_{}(self)\n", raw_name.to_lowercase()).as_bytes()).unwrap();
    file.write_all(b"    }\n").unwrap();
    file.write_all(b"}\n\n").unwrap();
}

fn write_visitor_trait(file: &mut LineWriter<File>, ast: &HashMap<String, Vec<(String, String)>>) {
    file.write_all(b"pub trait Visitor<T> {\n").unwrap();
    for (name, _) in ast {
        let raw_name = name.split('<').next().unwrap();
        file.write_all(format!("    fn visit_{}(&self, expr: &{}) -> T;\n", raw_name.to_lowercase(), name).as_bytes()).unwrap();
    }
    file.write_all(b"}\n\n").unwrap();
}