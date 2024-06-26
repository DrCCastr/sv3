#![feature(arbitrary_self_types)]

use std::collections::HashMap;

use crate::ast::{AssignmentExpr, BinaryExpr, Expr, Identifier, NodeType, NumericLiteral, ObjectLiteral, Program, Property, Stmt, VarDeclaration};
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
        match self.at().type_ {
            TokenType::Const | TokenType::Let => {
                self.parse_var_declaration()
            }
            _ => {
                self.parse_expr().into_boxed_stmt()
            }
        }
    }
    
    fn parse_var_declaration(&mut self) -> Box<dyn Stmt> {
        let is_constant = self.eat().type_ == TokenType::Const;
        let identifier = self.expect(
            TokenType::Identifier,
            "Expected identifier name following let | const keywords."
        ).value.clone();

        if self.at().type_ == TokenType::Semicolon {
            self.eat(); // Consome o ponto e vírgula
            if is_constant {
                panic!("Must assign value to constant expression. No value provided.");
            }

            return Box::new(VarDeclaration {
                kind: NodeType::VarDeclarationStmt,
                identifier,
                value: None,
                constant: false,
            });
        }

        self.expect(
            TokenType::Equals,
            "Expected equals token following identifier in var declaration."
        );

        let declaration = VarDeclaration {
            kind: NodeType::VarDeclarationStmt,
            identifier,
            value: Some(self.parse_expr()),
            constant: is_constant,
        };

        self.expect(
            TokenType::Semicolon,
            "Variable declaration statement must end with semicolon."
        );
        
        Box::new(declaration)
    }
    
    fn parse_assignment_expr(&mut self) -> Box<dyn Expr> {
        let left = self.parse_object_literal() as Box<dyn Expr>;
        
        if self.at().type_ == TokenType::Equals {
            self.eat();
            let value = self.parse_assignment_expr();
            return Box::new(AssignmentExpr {kind: NodeType::AssignmentExpr, assgine: left, value: value})
        }
        
        left
    }
    
    fn parse_object_literal(&mut self) -> Box<dyn Expr> {
        if self.at().type_ != TokenType::OpenBrace {
            return self.parse_additive_expr();
        }
        
        self.eat(); // Avançar após a chave aberta
        let mut properties: HashMap<String, Property> = HashMap::new();
        
        while self.not_eof() && self.at().type_ != TokenType::CloseBrace {
            let key = self.expect(TokenType::Identifier, "Object literal key expected").value;
            
            if self.at().type_ == TokenType::Comma {
                self.eat(); // Avançar após a vírgula
                properties.insert(key.clone(), Property { kind: NodeType::Property, key, value: None });
                return Box::new(ObjectLiteral { kind: NodeType::ObjectLiteral, value: properties });
            } else if self.at().type_ == TokenType::CloseBrace {
                properties.insert(key.clone(), Property { kind: NodeType::Property, key, value: None });
                return Box::new(ObjectLiteral { kind: NodeType::ObjectLiteral, value: properties });
            }
            
            self.expect(TokenType::Colon, "Missing colon following identifier in ObjectExpr");
            let value = self.parse_expr();
            
            properties.insert(key.clone(), Property { kind: NodeType::Property, key, value: Some(value) });
            
            if self.at().type_ != TokenType::CloseBrace {
                self.expect(TokenType::Comma, "Expected comma or closing bracket following property");
            }
        }
        
        self.expect(TokenType::CloseBrace, "Object literal missing closing brace.");
        Box::new(ObjectLiteral { kind: NodeType::ObjectLiteral, value: properties })
    }
    
    
    fn parse_expr(&mut self) -> Box<dyn Expr> {
        self.parse_assignment_expr()
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
