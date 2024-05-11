mod frontend;
use frontend::*;

use std::fs::File;
use std::io::Read;
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();
    File::open(&args[1]).expect("").read_to_string(&mut content);
    let tokens = frontend::parse::Parser::new().produce_ast(&content.to_string());
    println!("{:#?}", tokens);
}