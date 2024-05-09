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
    
    fn eat(&mut self) -> Token {
        let prev = self.tokens.remove(0);
        prev
    }
    
    fn parse_primary_expr(&mut self) -> Stmt {
        let ts = self.tokens[0].type_.clone();
        if ts == TokenType::Identifier {
            return Identifier::new(self.eat().value.clone());
        } else if ts == TokenType::Number {
            let number: f64  = self.eat().value.clone().parse().expect("f64");
            return NumericLiteral::new(number);
        }
        return Stmt::default();
    }
    
    fn parse_expr(&mut self) -> Stmt {
        self.parse_primary_expr()
    }
    
    fn parse_stmt(&mut self) -> Stmt {
        // skip
        self.parse_expr()
    }
    
    pub fn produceAST(&mut self, sourceCode: String) -> Stmt {
        self.tokens = tokenize(&sourceCode);
        let mut program = Program::new(Vec::new());
        
        while self.not_eof() {
            self.parse_stmt();
        }
        
        program
    }
}