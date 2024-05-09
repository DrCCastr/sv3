use std::vec::Vec;

#[derive(Debug)]
pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: NodeType,
    pub numeral_literal_value: f64,
    pub program_body: Vec<Stmt>,
    pub binary_expr_left: Expr,
    pub binary_expr_right: Expr,
    pub binary_expr_operator: String,
    pub identifier_symbol: String,
    pub numeric_literal_value: f64,
}

impl Stmt {
    pub fn new(kind: NodeType, numeral_literal_value: f64, program_body: Vec<Stmt>, binary_expr_left: Expr, binary_expr_right: Expr, binary_expr_operator: String, identifier_symbol: String, numeric_literal_value: f64) -> Self {
        Stmt {
            kind,
            numeral_literal_value,
            program_body,
            binary_expr_left,
            binary_expr_right,
            binary_expr_operator,
            identifier_symbol,
            numeric_literal_value,
        }
    }

    pub fn default() -> Self {
        Stmt {
            kind: NodeType::Program,
            numeral_literal_value: 0.0,
            program_body: Vec::new(),
            binary_expr_left: Expr,
            binary_expr_right: Expr,
            binary_expr_operator: String::new(),
            identifier_symbol: String::new(),
            numeric_literal_value: 0.0,
        }
    }
}

pub struct Program;

impl Program {
    pub fn new(body: Vec<Stmt>) -> Stmt {
        Stmt::new(NodeType::Program, 0.0, body, Expr, Expr, String::new(), String::new(), 0.0)
    }

    pub fn default() -> Stmt {
        Stmt::default()
    }
}

#[derive(Debug)]
pub struct Expr;

impl Expr {
    pub fn new() -> Stmt {
        Stmt::new(NodeType::BinaryExpr, 0.0, Vec::new(), Expr, Expr, String::new(), String::new(), 0.0)
    }

    pub fn default() -> Stmt {
        Stmt::default()
    }
}

pub struct BinaryExpr;

impl BinaryExpr {
    pub fn new(left: Expr, right: Expr, operator: String) -> Stmt {
        Stmt::new(NodeType::BinaryExpr, 0.0, Vec::new(), left, right, operator, String::new(), 0.0)
    }

    pub fn default() -> Stmt {
        Stmt::default()
    }
}

pub struct Identifier;

impl Identifier {
    pub fn new(symbol: String) -> Stmt {
        Stmt::new(NodeType::Identifier, 0.0, Vec::new(), Expr, Expr, String::new(), symbol, 0.0)
    }

    pub fn default() -> Stmt {
        Stmt::default()
    }
}

pub struct NumericLiteral;

impl NumericLiteral {
    pub fn new(value: f64) -> Stmt {
        Stmt::new(NodeType::NumericLiteral, value, Vec::new(), Expr, Expr, String::new(), String::new(), value)
    }

    pub fn default() -> Stmt {
        Stmt::default()
    }
}
