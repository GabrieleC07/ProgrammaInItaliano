use crate::lexer::tokens::Operator;


#[derive(Debug)]
pub enum NodeStmt {
    Return(NodeExpr),
    VarDecl(String, NodeExpr),
}

#[derive(Debug)]
pub enum NodeExpr {
    IntLiteral(isize),
    Identifier(String),
    MathOperat(NodeMathExpr)
}
#[derive(Debug)]
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
