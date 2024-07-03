use crate::lexer::tokens::Operator;


#[derive(Debug, Clone)]
pub enum NodeStmt {
    Return(NodeExpr),
    VarDecl(String, NodeExpr),
    Scope(Vec<NodeStmt>),
}

#[derive(Debug, Clone)]
pub enum NodeExpr {
    IntLiteral(isize),
    Identifier(String),
    MathOperat(NodeMathExpr)
}
#[derive(Debug, Clone)]
pub struct NodeMathExpr {
    pub left_side: Box<NodeExpr>,
    pub operator: Operator,
    pub right_side: Box<NodeExpr>,
}
impl NodeMathExpr {
    pub fn new(left: NodeExpr, operator: Operator, right: NodeExpr) -> NodeMathExpr {
        NodeMathExpr {
            left_side: Box::new(left),
            operator,
            right_side: Box::new(right)
        }
    }
}
