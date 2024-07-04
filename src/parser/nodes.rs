use crate::lexer::tokens::Operator;

#[derive(Debug, Clone)]
pub enum NodeStmt {
    Return(NodeExpr),
    VarDecl(String, NodeExpr),
    VarShadowing(String, NodeExpr),
    Scope(Vec<NodeStmt>),
    Functions(BuiltInFunctions),
}

#[derive(Debug, Clone)]
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
}
#[derive(Debug, Clone)]
pub struct NodeMathExpr {
    pub left_expr: Box<NodeExpr>,
    pub operator: Operator,
    pub right_expr: Box<NodeExpr>,
}