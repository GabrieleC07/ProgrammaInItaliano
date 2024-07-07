use crate::lexer::tokens::Operator;

#[derive(Debug, Clone)]
pub enum NodeStmt {
    Return(NodeExpr),
    VarDecl(String, VarDeclTypes),
    VarShadowing(String, NodeExpr),
    Scope(Vec<NodeStmt>),
    CompilerBuiltInFunctions(BuiltInFunctions),
    FnCall(NodeFnCall)
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum VarDeclTypes {
    Expr(NodeExpr),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum NodeExpr {
    IntLiteral(isize),
    Identifier(String),
    MathOperat(NodeMathExpr)
}
#[derive(Debug, Clone)]
pub enum BuiltInFunctions {
    If(Box<NodeIfStmt>),
    While(Box<NodeWhileStmt>),
}


#[derive(Debug, Clone)]
pub struct NodeIfStmt {
    pub scope: NodeStmt,
    pub condition: NodeEquality,
}
#[derive(Debug, Clone)]
pub struct NodeWhileStmt {
    pub scope: NodeStmt,
    pub condition: NodeEquality,
}
#[derive(Debug, Clone)]
pub struct NodeEquality {
    pub right_expr: NodeExpr,
    pub left_expr: NodeExpr,
    pub is_inequality: bool
}
#[derive(Debug, Clone)]
pub struct NodeFnCall {
    pub name: String,
    pub is_built_in: bool,
    pub argument: String,
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct NodeMathExpr {
    pub left_expr: Box<NodeExpr>,
    pub operator: Operator,
    pub right_expr: Box<NodeExpr>,
}