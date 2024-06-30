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
    fn match_token(&self, kind: &TokenType) -> bool {
        &self.current_token().kind == kind
    }
    
    pub fn parse_prog(&mut self) -> Result<Vec<NodeStmt>, String> {
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

            if let TokenType::Ident(name) = self.current_token().kind {
                println!("Entered ident");
                self.advance(); // Consume Ident

                if self.match_token(&TokenType::Eq) {
                    self.advance(); // Consume '='
                    println!("Entered eq");

                    let node_expr = self.parse_node_expr()?;
                    return Ok(NodeStmt::VarDecl(name, node_expr));
                }
            }
            return Err(String::from("Expected an identifier after 'var'"));
        }
        Err(String::from(format!("Unexpected token {:?}", self.current_token())))
    }

    fn parse_node_expr(&mut self) -> Result<NodeExpr, String> {
        if let TokenType::IntLit(value) = self.current_token().kind {
            let right_side_expr: NodeExpr;
            self.advance(); // Consume integer literal

            if let TokenType::Operators(operator) = self.current_token().kind {
                self.advance(); // Consume operator
                if let TokenType::IntLit(value) = self.current_token().kind {
                    self.advance(); // Consume int_lit
                    right_side_expr = NodeExpr::IntLiteral(value)
                }
                else if let TokenType::Ident(name) = self.current_token().kind {
                    self.advance(); // Consume ident
                    right_side_expr = NodeExpr::Identifier(name)
                }
                else {
                    return Err(String::from("Expected either an IntLit or an Ident"));
                }
                let left_side_expr = NodeExpr::IntLiteral(value);
                return Ok(NodeExpr::MathOperat(Box::new(left_side_expr), operator, Box::new(right_side_expr)))
            }
            return Ok(NodeExpr::IntLiteral(value));
        }
        else if let TokenType::Ident(name) = self.current_token().kind {
            let right_side_expr: NodeExpr;
            self.advance(); // Consume ident

            if let TokenType::Operators(operator) = self.current_token().kind {
                self.advance(); // Consume operator
                if let TokenType::IntLit(value) = self.current_token().kind {
                    self.advance(); // Consume int_lit
                    right_side_expr = NodeExpr::IntLiteral(value)
                }
                else if let TokenType::Ident(name) = self.current_token().kind {
                    self.advance(); // Consume ident
                    right_side_expr = NodeExpr::Identifier(name)
                }
                else {
                    return Err(String::from("Expected either an IntLit or an Ident"));
                }
                let left_side_expr = NodeExpr::Identifier(name);
                return Ok(NodeExpr::MathOperat(Box::new(left_side_expr), operator, Box::new(right_side_expr)))
            }
            return Ok(NodeExpr::Identifier(name));
        }
        Err(String::from("Unexpected token in NodeExpression"))
    }
}