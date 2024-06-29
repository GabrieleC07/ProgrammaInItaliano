#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum TokenType {
    Ret,
    IntLit(isize),
    OpenParen,
    ClosedParen,
    Ident(String),
    Var,
    Eq,
    EoF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    // pub value: Option<i32>,
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
            token_type
        }
    }
}