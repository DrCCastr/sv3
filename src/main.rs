mod frontend;
use frontend::*;

use std::fs::File;
use std::io::Read;
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pars = parse::parser::New();
    let mut input = String::new();
    while input.clone() != "exit" {
        stdin().read_line(&mut input).expect("Falha ao ler a linha");
        println!("{:#?}", pars.produceAST(input.clone()));
    }
}