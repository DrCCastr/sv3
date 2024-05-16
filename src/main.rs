mod frontend;
mod runtime;
use frontend::ast::Stmt;
use frontend::*;
use runtime::interpreter::evaluate;
use runtime::value::SunVariable;
use runtime::*;

use std::fs::File;
use std::io::Read;
use std::env;
use std::io::stdin;

use crate::environmment::Environmment;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();
    File::open(&args[1]).expect("").read_to_string(&mut content);
    let tokens = frontend::parse::Parser::new().produce_ast(&content.to_string());
    //println!("{:#?}", tokens);
    if let Some(programToken) = tokens.as_program() {
        println!("{:#?}", evaluate(programToken,  Environmment::new()));
    }
}