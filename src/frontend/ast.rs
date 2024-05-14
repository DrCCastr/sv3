use std::fmt;

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    NumericLiteral,
    NilLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt: fmt::Debug {
    fn get_kind(&self) -> NodeType;
    fn as_numeric_literal(&self) -> Option<&NumericLiteral> {
        None
    }
    fn as_binary_expr(&self) -> Option<&BinaryExpr> {
        None
    }
}

pub trait Expr: Stmt {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt>;
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program {{\nkind: {:?},\nbody: {:#?} }}\n", self.kind, self.body)
    }
}

pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: String,
}

impl fmt::Debug for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinaryExpr {{\n\tkind: {:?}\n\tleft: {:#?}\n\tright: {:#?}\n\toperator: {:?} \n}}\n",
            self.kind, self.left, self.right, self.operator
        )
    }
}

pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier {{\tkind: {:?},\tsymbol: {:?}}}\n", self.kind, self.symbol)
    }
}

pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: f64,
}

impl fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NumericLiteral {{\tkind: {:?},\tvalue: {:?} }}\n", self.kind, self.value)
    }
}

pub struct NilLiteral {
    pub kind: NodeType,
    pub value: String
}

impl fmt::Debug for NilLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NilLiteral {{kind: {:?}, value: {:?}}}", self.kind, self.value)
    }
}

impl Stmt for Program {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
}
impl Stmt for BinaryExpr {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_binary_expr(&self) -> Option<&BinaryExpr> {
        Some(self)
    }
}
impl Stmt for Identifier {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
}
impl Stmt for NumericLiteral {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_numeric_literal(&self) -> Option<&NumericLiteral> {
        Some(self)
    }
}
impl Stmt for NilLiteral {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
}

impl Expr for NilLiteral {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
}
impl Expr for BinaryExpr {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
}
impl Expr for Identifier {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
}
impl Expr for NumericLiteral {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
}
