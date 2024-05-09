use std::vec::Vec;

pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr
}

pub struct Stmt {
    pub kind: NodeType
}

impl Stmt {
    pub fn New(kind: NodeType) -> Self {
        Self {
            kind: kind
        }
    }
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>
}

impl Program {
    pub fn New(body: Vec<Stmt>) -> Self {
        Self {
            kind: NodeType::Program,
            body: body
        }
    }
}

pub struct Expr {
    pub kind: NodeType
}

impl Expr {
    pub fn New() -> Self {
        Self {
            kind: NodeType::BinaryExpr
        }
    }
}

pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Expr,
    pub right: Expr,
    pub operator: String
}

impl BinaryExpr {
    pub fn New(left: Expr, right: Expr, operator: String) -> Self {
        Self {
            kind: NodeType::BinaryExpr,
            left: left,
            right: right,
            operator: operator
        }
    }
}

pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String
}

impl Identifier {
    pub fn New(symbol: String) -> Self {
        Self {
            kind: NodeType::Identifier,
            symbol: symbol
        }
    }
}

pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: i64
}

impl NumericLiteral {
    fn New(value: i64) -> Self {
        Self {
            kind: NodeType::NumericLiteral,
            value: value
        }
    }
}
