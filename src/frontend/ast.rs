use std::{collections::HashMap, fmt, iter::Map};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    // Statements
    Program,
    VarDeclarationStmt,
    // Expressions
    AssignmentExpr,
    // Literals
    Property,
    ObjectLiteral,
    NumericLiteral,
    StringLiteral,
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
    fn as_program(&self) -> Option<&Program> {
        None
    }
    fn as_identifier(&self) -> Option<&Identifier> {
        None
    }
    fn as_var_declaration(&self) -> Option<&VarDeclaration> {
        None
    }
    fn as_assignment_expr(&self) -> Option<&AssignmentExpr> {
        None
    }
    fn as_property(&self) -> Option<&Property> {
        None
    }
    fn as_object_literal(&self) -> Option<&ObjectLiteral> {
        None
    }
}

pub trait Expr: Stmt {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt>;
    fn as_stmt(&self) -> &dyn Stmt;
}

pub struct VarDeclaration {
    pub kind: NodeType,
    pub constant: bool,
    pub identifier: String,
    pub value: Option<Box<dyn Expr>>,
}

impl fmt::Debug for VarDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{kind: {:?}, constant: {:?}, iden:ifier: {:?}, value: {:#?}}}", self.kind, self.constant, self.identifier, self.value)
    }
}

pub struct AssignmentExpr {
    pub kind: NodeType,
    pub assgine: Box<dyn Expr>,
    pub value: Box<dyn Expr>
}

impl fmt::Debug for AssignmentExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n\tkind: {:?}\n\tassgine: {:#?}\n\tvalue: {:#?}\n}}", self.kind, self.assgine, self.value)
    }
}

pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Box<dyn Stmt>>,
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\nkind: {:?},\nbody: {:#?} }}\n", self.kind, self.body)
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
            "{{\n\tkind: {:?}\n\tleft: {:#?}\n\tright: {:#?}\n\toperator: {:?} \n}}\n",
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
        write!(f, "{{\tkind: {:?},\tsymbol: {:?}}}\n", self.kind, self.symbol)
    }
}

pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: f64,
}

impl fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\tkind: {:?},\tvalue: {:?} }}\n", self.kind, self.value)
    }
}

pub struct Property {
    pub kind: NodeType,
    pub key: String,
    pub value: Option<Box<dyn Expr>>
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\nkind: {:?}\nKey: {},\nValue: {:?} }}", self.kind, self.key, self.value)
    }
}

pub struct ObjectLiteral {
    pub kind: NodeType,
    pub value: HashMap<String, Property>,
}

impl fmt::Debug for ObjectLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\nkind: {:?}, \nvalue: {:?}}}", self.kind, self.value)
    }
}

impl Stmt for AssignmentExpr {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_assignment_expr(&self) -> Option<&AssignmentExpr> {
        Some(self)
    }
}
impl Stmt for VarDeclaration {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_var_declaration(&self) -> Option<&VarDeclaration> {
        Some(self)
    }
}
impl Stmt for Program {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_program(&self) -> Option<&Program> {
        Some(self)
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
    fn as_identifier(&self) -> Option<&Identifier> {
        Some(self)
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
impl Stmt for ObjectLiteral {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_object_literal(&self) -> Option<&ObjectLiteral> {
        Some(self)
    }
}
impl Stmt for Property {
    fn get_kind(&self) -> NodeType {
        self.kind.clone()
    }
    fn as_property(&self) -> Option<&Property> {
        Some(self)
    }
}

impl Expr for AssignmentExpr {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
impl Expr for BinaryExpr {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
impl Expr for Identifier {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
impl Expr for NumericLiteral {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
impl Expr for ObjectLiteral {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
impl Expr for Property {
    fn into_boxed_stmt(self: Box<Self>) -> Box<dyn Stmt> {
        self
    }
    fn as_stmt(&self) -> &dyn Stmt {
        self
    }
}
