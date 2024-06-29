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

    // Helper to get the current token
    fn current_token(&self) -> Token {
        self.tokens.get(self.current).cloned().unwrap_or_else(|| Token::new(TokenType::EoF)).clone()
    }

    // Helper to advance to the next token
    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    // Helper to check the current token type
    fn match_token(&self, token_type: &TokenType) -> bool {
        &self.current_token().token_type == token_type
    }
    
    pub fn parse(&mut self) -> Result<Vec<NodeStmt>, String> {
        let mut stmts = Vec::new();
        while !self.match_token(&TokenType::EoF) {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<NodeStmt, String> {
        if self.match_token(&TokenType::Ret) {
            self.advance(); // Consume 'return'

            if self.match_token(&TokenType::OpenParen) {
                self.advance(); // Consume '('

                let node_expr = self.parse_node_expr()?;
                if self.match_token(&TokenType::ClosedParen) {
                    self.advance(); // Consume ')'

                    return Ok(NodeStmt::Return(node_expr));
                } 
                return Err(String::from("Expected '('"));
            } 
            return Err(String::from("Expected ')'"));
        }
        else if self.match_token(&TokenType::Var) {
            self.advance(); // Consume 'var'

            if let TokenType::Ident(name) = self.current_token().token_type.clone() {
                self.advance(); // consume Ident

                if self.match_token(&TokenType::Eq) {
                    self.advance(); // consume '='

                    let node_expr = self.parse_node_expr()?;
                    return Ok(NodeStmt::VarDecl(name, node_expr));
                }
                return Err(String::from("Expected '='"));
                
            }
            return Err(String::from("Expected an identifier after 'var'"));
        }
        Err(String::from("Unexpected token"))
    }

    fn parse_node_expr(&mut self) -> Result<NodeExpr, String> {
        if let TokenType::IntLit(value) = self.current_token().token_type {
            self.advance(); // consume integer literal
            return Ok(NodeExpr::IntLiteral(value));  
        }
        else if let TokenType::Ident(ref name) = self.current_token().token_type {
            self.advance(); // consume identifier
            return Ok(NodeExpr::Identifier(name.clone()));
        }

        Err(String::from("Unexpected token in NodeExpression"))
    }
}