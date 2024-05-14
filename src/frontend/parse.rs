#![feature(arbitrary_self_types)]

use crate::ast::{BinaryExpr, Expr, Identifier, NilLiteral, NodeType, NumericLiteral, Program, Stmt};
use crate::lexer::{tokenize, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { tokens: Vec::new() }
    }

    fn not_eof(&self) -> bool {
        !self.tokens.is_empty() && self.tokens[0].type_ != TokenType::EOF
    }

    fn at(&self) -> &Token {
        &self.tokens[0]
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, type_: TokenType, err: &str) -> Token {
        let prev = self.eat();
        if prev.type_ != type_ {
            eprintln!("Parser Error:\n{} {:?} - Expecting: {:?}", err, prev, type_);
            std::process::exit(1);
        }

        prev
    }

    pub fn produce_ast(&mut self, source_code: &str) -> Program {
        self.tokens = tokenize(source_code);
        let mut program = Program { kind: NodeType::Program, body: Vec::new() };

        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }
        
        program
    }

    fn parse_stmt(&mut self) -> Box<dyn Stmt> {
        self.parse_expr().into_boxed_stmt()
    }
    
    fn parse_expr(&mut self) -> Box<dyn Expr> {
        self.parse_additive_expr() as Box<dyn Expr>
    }
    
    fn parse_additive_expr(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_multiplicative_expr();
    
        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value.clone();
            let right = self.parse_multiplicative_expr();
            left = Box::new(BinaryExpr { kind: NodeType::BinaryExpr, left, right, operator });
        }
    
        left
    }
    

    fn parse_multiplicative_expr(&mut self) -> Box<dyn Expr> {
        let mut left = self.parse_primary_expr();

        while self.at().value == "/" || self.at().value == "*" || self.at().value == "%" {
            let operator = self.eat().value.clone();
            let right = self.parse_primary_expr();
            left = Box::new(BinaryExpr { kind: NodeType::BinaryExpr, left, right, operator });
        }

        left
    }

    fn parse_primary_expr(&mut self) -> Box<dyn Expr> {
        match self.at().type_ {
            TokenType::Identifier => {
                let symbol = self.eat().value.clone();
                Box::new(Identifier { kind: NodeType::Identifier, symbol })
            }
            TokenType::Nil => {
                self.eat();
                Box::new(NilLiteral {kind: NodeType::NilLiteral, value: "a".to_string()})
            }
            TokenType::Number => {
                let value = self.eat().value.parse().expect("Failed to parse number");
                Box::new(NumericLiteral { kind: NodeType::NumericLiteral, value })
            }
            TokenType::OpenParen => {
                self.eat();
                let value = self.parse_expr();
                self.expect(TokenType::CloseParen, "Unexpected token found inside parenthesised expression. Expected closing parenthesis.");
                value
            }
            _ => {
                println!("Unexpected token found during parsing! {:?}", self.at());
                Box::new(Identifier { kind: NodeType::Identifier, symbol: self.eat().value })
            }
        }
    }
}
