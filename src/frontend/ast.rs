pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt {}

pub trait Expr: Stmt {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt>;
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}

pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: String,
}

pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: i32,
}

impl Stmt for Program {}
impl Stmt for BinaryExpr {}
impl Stmt for Identifier {}
impl Stmt for NumericLiteral {}
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