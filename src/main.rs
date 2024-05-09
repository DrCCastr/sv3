mod lexer;
use lexer::lexer::*;

fn main() {
    println!("{:?}", tokenize( "let x = 1"));
}