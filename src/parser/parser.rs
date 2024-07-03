use crate::lexer::tokens::*;

use crate::parser::nodes::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn current_token(&self, offset: isize) -> Option<Token> {
        self.tokens.get((self.current as isize + offset) as usize).cloned()
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn match_token(&self, kind: &TokenType) -> bool {
        let current_token = self.current_token(0);
        
        if current_token.is_none() {
            return false;
        }
        else if current_token.unwrap().kind == kind.clone() {
            return true;
        }
        false
    }
    
    pub fn parse_prog(&mut self) -> Result<Vec<NodeStmt>, String> {
        let mut stmts = Vec::new();
        while self.current < self.tokens.len() {
            let stmt_parsed = self.parse_stmt()?;
            stmts.push(stmt_parsed);
        }
        println!("Stmts: {:?}", stmts);
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<NodeStmt, String> {
        if self.match_token(&TokenType::Ret) {
            self.advance();
            if self.match_token(&TokenType::OpenParen) {
                self.advance();
                let node_expr = self.parse_node_expr()?;
                if self.match_token(&TokenType::ClosedParen) {
                    self.advance();
                    return Ok(NodeStmt::Return(node_expr));
                }
                return Err(String::from("Expected ')'"));
            }
            return Err(String::from("Expected '('"));
        } 
        else if self.match_token(&TokenType::Var) {
            self.advance();
            if let Some(Token { kind: TokenType::Ident(name), .. }) = self.current_token(0) {
                self.advance();
                if self.match_token(&TokenType::Eq) {
                    self.advance();
                    let node_expr = self.parse_node_expr()?;
                    return Ok(NodeStmt::VarDecl(name, node_expr));
                }
            }
            return Err(String::from("Expected an identifier after 'var'"));
        } 
        else if self.match_token(&TokenType::OpenCurlyBracket) {
            return self.parse_scope();
        }
        Err(format!("Unexpected token {:?}", self.current_token(0)))
    }
    

    fn parse_node_expr(&mut self) -> Result<NodeExpr, String> {
        if let Some(Token { kind: TokenType::IntLit(value), .. }) = self.current_token(0) {
            self.advance();
            if let Some(Token { kind: TokenType::Operators(_), .. }) = self.current_token(0) {
                return self.parse_math_expr(NodeExpr::IntLiteral(value));
            }
            return Ok(NodeExpr::IntLiteral(value));
        } 
        else if let Some(Token { kind: TokenType::Ident(name), .. }) = self.current_token(0) {
            self.advance();
            if let Some(Token { kind: TokenType::Operators(_), .. }) = self.current_token(0) {
                return self.parse_math_expr(NodeExpr::Identifier(name));
            }
            return Ok(NodeExpr::Identifier(name));
        }
        Err(format!("Unexpected: {:?}", self.current_token(0)))
    }
    

    fn parse_math_expr(&mut self, left_expr: NodeExpr) -> Result<NodeExpr, String> {
        if let Some(Token { kind: TokenType::Operators(operator), .. }) = self.current_token(0){
            self.advance(); // Consume the operator
            
            let right_expr = self.parse_node_expr()?;
            Ok(NodeExpr::MathOperat(NodeMathExpr::new(left_expr, operator, right_expr)))
        } else {
            Err(format!("Expected an operator, found {:?}", self.current_token(0)))
        }
    }
    fn parse_scope(&mut self) -> Result<NodeStmt, String> {
        self.advance(); // Consume '{'
        let mut stmts: Vec<NodeStmt> = Vec::new();

        while !self.match_token(&TokenType::ClosedCurlyBracket) {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        
        if self.match_token(&TokenType::ClosedCurlyBracket) {
            self.advance(); // Consume '}'
            return Ok(NodeStmt::Scope(stmts));
        }
        Err("Did not find any stmts in scope".to_string())
    }
}