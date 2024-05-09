use super::ast::*;
use super::lexer;
use super::lexer::*;

use std::vec::Vec;

pub struct parser {
    pub tokens: Vec<Token>
}

impl parser {
    pub fn New() -> Self {
        Self {
            tokens: Vec::new()
        }
    }
    
    fn not_eof(&mut self) -> bool {
        self.tokens[0].type_ != TokenType::EOF
    }
    
    pub fn produceAST(&mut self, sourceCode: String) -> Program {
        self.tokens = tokenize(&sourceCode);
        let mut program = Program::New(Vec::new());
        
        program
    }
}