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

    fn current_token(&self, offset: usize) -> Token {
        return self.tokens.get(self.current + offset).cloned().unwrap_or_else(|| Token::new(TokenType::EoF)).clone();
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn match_token(&self, kind: &TokenType) -> bool {
        &self.current_token(0).kind == kind
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

            if let TokenType::Ident(name) = self.current_token(0).kind {
                self.advance(); // Consume Ident

                if self.match_token(&TokenType::Eq) {
                    self.advance(); // Consume '='

                    let node_expr = self.parse_node_expr()?;
                    return Ok(NodeStmt::VarDecl(name, node_expr));
                }
            }
            return Err(String::from("Expected an identifier after 'var'"));
        }
        Err(String::from(format!("Unexpected token {:?}", self.current_token(0))))
    }

    fn parse_node_expr(&mut self) -> Result<NodeExpr, String> {
        if let TokenType::IntLit(value) = self.current_token(0).kind {
            self.advance(); // Consume the literal

            if let TokenType::Operators(_) = self.current_token(0).kind {
                return self.parse_math_expr(NodeExpr::IntLiteral(value));
            } else {
                return Ok(NodeExpr::IntLiteral(value));
            }

        } else if let TokenType::Ident(name) = self.current_token(0).kind {
            self.advance(); // Consume the identifier

            if let TokenType::Operators(_) = self.current_token(0).kind {
                return self.parse_math_expr(NodeExpr::Identifier(name));
            } else {
                return Ok(NodeExpr::Identifier(name));
            }
        } else {
            return Err(format!("Unexpected token: {:?}", self.current_token(0)));
        }
    }

    fn parse_math_expr(&mut self, left_expr: NodeExpr) -> Result<NodeExpr, String> {
        if let TokenType::Operators(op) = self.current_token(0).kind {
            self.advance(); // Consume the operator
            
            let right_expr = self.parse_node_expr()?;
            Ok(NodeExpr::MathOperat(NodeMathExpr::new(left_expr, op, right_expr)))
        } else {
            Err(format!("Expected an operator, found {:?}", self.current_token(0)))
        }
    }
}