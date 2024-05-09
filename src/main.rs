mod lexer;
use lexer::lexer::*;

use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();
    File::open(&args[1]).expect("").read_to_string(&mut content);
    let tokens = tokenize(&content);
    print!("{:#?}", tokens);
}