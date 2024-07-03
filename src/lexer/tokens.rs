#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Ret,
    IntLit(isize),
    OpenParen,
    ClosedParen,
    Ident(String),
    Var,
    Eq,
    Operators(Operator),
    Fn,
    OpenCurlyBracket,
    ClosedCurlyBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}
impl Token {
    pub fn new(kind: TokenType) -> Token {
        Token {
            kind
        }
    }
}