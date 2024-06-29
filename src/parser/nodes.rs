#[derive(Debug)]
pub enum NodeStmt {
    Return(NodeExpr),
    VarDecl(String, NodeExpr),
}

#[derive(Debug)]
pub enum NodeExpr {
    IntLiteral(isize),
    Identifier(String),
}