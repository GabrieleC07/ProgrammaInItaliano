use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Ret,
    IntLit(isize),
    OpenParen,
    ClosedParen,
    Ident(String),
    String(String),
    Var,
    Eq,
    Fn,
    OpenCurlyBracket,
    ClosedCurlyBracket,
    If,
    While,
    Operators(Operator),
    True,
    False,
    ExclamationPoint,
    PrintFn,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    FSlash,
}

impl Token {
    pub fn new(kind: TokenType) -> Token {
        Token {
            kind
        }
    }
}


pub fn create_keyword_map() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();

    map.insert(String::from("ritorna"), TokenType::Ret);
    map.insert(String::from("("), TokenType::OpenParen);
    map.insert(String::from(")"), TokenType::ClosedParen);
    map.insert(String::from("var"), TokenType::Var);
    map.insert(String::from("="), TokenType::Eq);
    map.insert(String::from("+"), TokenType::Operators(Operator::Plus));
    map.insert(String::from("-"), TokenType::Operators(Operator::Minus));
    map.insert(String::from("*"), TokenType::Operators(Operator::Star));
    map.insert(String::from("/"), TokenType::Operators(Operator::FSlash));
    map.insert(String::from("fn"), TokenType::Fn);
    map.insert(String::from("{"), TokenType::OpenCurlyBracket);
    map.insert(String::from("}"), TokenType::ClosedCurlyBracket);
    map.insert(String::from("se"), TokenType::If);
    map.insert(String::from("vero"), TokenType::True);
    map.insert(String::from("falso"), TokenType::False);
    map.insert(String::from("finché"), TokenType::While);
    map.insert(String::from("!"), TokenType::ExclamationPoint);
    map.insert(String::from("stampa"), TokenType::PrintFn);
    
    map
}