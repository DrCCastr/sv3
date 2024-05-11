use std::fmt;

#[derive(Debug)]
pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub trait Stmt: fmt::Debug {}

pub trait Expr: Stmt {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt>;
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program {{\nkind: {:#?},\nbody: {:#?} }}\n", self.kind, self.body)
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
            "BinaryExpr {{\n\tkind: {:#?},\n\tleft: {:#?},\n\tright: {:#?},\n\toperator: {:#?} \n}}\n",
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
        write!(f, "Identifier {{\n\tkind: {:#?},\n\tsymbol: {:#?} \n}}\n", self.kind, self.symbol)
    }
}

pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: i32,
}

impl fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NumericLiteral {{\n\tkind: {:#?},\n\tvalue: {:#?} \n}}\n", self.kind, self.value)
    }
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
