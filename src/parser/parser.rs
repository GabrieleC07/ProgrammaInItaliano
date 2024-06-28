use crate::lexer::lexer::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> Option<NodeExit> {
    let mut tokens_clone = tokens.clone();

    let result = parse_node_exit(&mut tokens_clone);

    result
}

fn parse_node_exit(tokens: &mut Vec<Token>) -> Option<NodeExit> {
    let mut tokens_clone = tokens.clone();

    if tokens_clone.is_empty() {
        return None;
    }

    let first_token = peek_token(0, &tokens_clone);
    if first_token.is_some() && first_token.unwrap().token_type != TokenType::Ret {
        eprintln!("Not Found Ret");
        return None;
    }
    consume_token(0, &mut tokens_clone);

    let open_paren_token = peek_token(0, &tokens_clone);
    if open_paren_token.is_some() && open_paren_token.unwrap().token_type != TokenType::OpenParen {
        eprintln!("Not Found OpenParen");
        return None;
    }
    consume_token(0, &mut tokens_clone);

    let int_lit_token = peek_token(0, &tokens_clone);
    if int_lit_token.is_some() && int_lit_token.unwrap().token_type != TokenType::IntLit {
        eprintln!("Not Found IntLit");
        return None;
    }
    let int_lit_token = consume_token(0, &mut tokens_clone);

    let closed_paren_token = peek_token(0, &tokens_clone);
    if closed_paren_token.is_some() && closed_paren_token.unwrap().token_type != TokenType::ClosedParen {
        eprintln!("Not Found ClosedParen");
        return None;
    }
    consume_token(0, &mut tokens_clone);

    // Update main tokens accordingly if succesful
    consume_token(0, tokens);
    consume_token(0, tokens);
    consume_token(0, tokens);
    consume_token(0, tokens);
    
    Some(
        NodeExit::new(
            NodeExpr::new(int_lit_token.unwrap())
        )
    )
}

fn consume_token(index: usize, tokens: &mut Vec<Token>) -> Option<Token> {
    if tokens.len() <= index {
        return None;
    }
    Some(tokens.remove(index))
}

fn peek_token(index: usize, tokens: &Vec<Token>) -> Option<Token> {
    if tokens.len() <= index {
        return None;
    }
    Some(tokens[index].clone())
}
#[derive(Debug, PartialEq)]
pub struct NodeExit {
    pub expr: NodeExpr
}

impl NodeExit {
    pub fn new(expr: NodeExpr) -> NodeExit {
        NodeExit {
            expr
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct NodeExpr {
    pub int_lit: Token
}
impl NodeExpr {
    pub fn new(int_lit: Token) -> NodeExpr {
        NodeExpr {
            int_lit
        }
    }
}