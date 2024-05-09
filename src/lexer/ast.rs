use std::vec::Vec;

pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr
}

pub struct Stmt {
    kind: NodeType
}

impl Stmt {
    pub fn New(kind: NodeType) -> Self {
        Self {
            kind: kind
        }
    }
}

pub struct Program {
    kind: NodeType,
    body: Vec<Stmt>
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
    kind: NodeType
}

impl Expr {
    pub fn New() -> Self {
        Self {
            kind: NodeType::BinaryExpr
        }
    }
}

pub struct BinaryExpr {
    kind: NodeType,
    left: Expr,
    right: Expr,
    operator: String
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
    kind: NodeType,
    symbol: String
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
    kind: NodeType,
    value: i64
}

impl NumericLiteral {
    fn New(value: i64) -> Self {
        Self {
            kind: NodeType::NumericLiteral,
            value: value
        }
    }
}
