use std::{clone, vec::Vec, collections::HashMap};

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen, CloseParen,
    BinaryOperator,
    Let,
    Nil,
    EOF
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub type_: TokenType
}

impl Token {
    pub fn new(value: String, type_: TokenType) -> Self {
        Self {
            value: value,
            type_: type_
        }
    }
}

fn token(value: &str, type_: TokenType) -> Token {
    Token { value: value.to_string(), type_ }
}

fn isalpha(src: &str) -> bool {
    src.to_uppercase() != src.to_lowercase()
}

fn isint(str: &str) -> bool {
    str.chars().all(|c| c.is_ascii_digit())
}

fn keyword(str: String) -> TokenType {
    if str == "let" {
        return TokenType::Let;
    }
    TokenType::Nil
}

fn isskipabble(str: &str) -> bool {
    return str == " " || str == "\n" || str == "\t"
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src: Vec<char> = source_code.chars().collect();
    while !src.is_empty() {
        if src[0] == '(' {
            tokens.push(token(&src.remove(0).to_string(), TokenType::OpenParen));
        } else if src[0] == ')' {
            tokens.push(token(&src.remove(0).to_string(), TokenType::CloseParen));
        }
        else if src[0] == '+' || src[0] == '-' || src[0] == '*' || src[0] == '/' {
            tokens.push(token(&src.remove(0).to_string(), TokenType::BinaryOperator));
        }
        else if src[0] == '=' {
            tokens.push(token(&src.remove(0).to_string(), TokenType::Equals));
        } else {
            if isint(&src[0].to_string()) {
                let mut num = String::new();
                while !src.is_empty() && isint(&src[0].to_string()) {
                    num.push(src.remove(0));
                }
                tokens.push(token(&num, TokenType::Number));
            } else if isalpha(&src[0].to_string()) {
                let mut ident = String::new();
                while !src.is_empty() && isalpha(&src[0].to_string()) {
                    ident.push(src.remove(0));
                }
                let reseverd = keyword(ident.clone());
                if reseverd != TokenType::Nil {
                    tokens.push(token(&ident, reseverd));
                } else {
                    tokens.push(token(&ident, TokenType::Identifier));
                }
            } else if isskipabble(&src[0].to_string()) {
                src.remove(0);
            } else {
                println!("Unreconized character found in code: {}", &src[0].to_string());
            }
        }
    }
    tokens.push(Token::new("EOF".to_string(), TokenType::EOF));
    tokens
}