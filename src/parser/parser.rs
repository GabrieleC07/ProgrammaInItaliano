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
        
        if current_token.is_some() && current_token.unwrap().kind == kind.clone() {
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
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<NodeStmt, String> {
        if self.match_token(&TokenType::Ret) {
            self.advance(); // Consume 'return'
            if self.match_token(&TokenType::OpenParen) {
                self.advance(); // Consume '('
                let node_expr = self.parse_expr()?;
                if self.match_token(&TokenType::ClosedParen) {
                    self.advance(); // Consume ')'
                    return Ok(NodeStmt::Return(node_expr));
                }
                return Err(String::from("Expected ')'"));
            }
            return Err(format!("Expected '(' found {:?}", self.current_token(0)));
        } 
        else if self.match_token(&TokenType::Var) {
            self.advance(); // Consume 'var'
            if let Some(Token { kind: TokenType::Ident(ident), .. }) = self.current_token(0) {
                self.advance();
                if self.match_token(&TokenType::Eq) {
                    self.advance();
                    let node_expr = self.parse_expr()?;
                    return Ok(NodeStmt::VarDecl(ident, node_expr));
                }
            }
            return Err(String::from("Expected an identifier after 'var'"));
        }
        else if let Some(Token { kind: TokenType::Ident(ident), .. }) = self.current_token(0) {
            self.advance(); // Consume 'ident'
            if self.match_token(&TokenType::Eq) {
                self.advance(); // Consume '='
                let expr = self.parse_expr()?;
                return Ok(NodeStmt::VarShadowing(ident, expr));
            }
            return Err("Expected '=' for Var Shadowing".to_string());
        }
        else if self.match_token(&TokenType::OpenCurlyBracket) {
            return self.parse_scope();
        }
        else if self.match_token(&TokenType::If) || self.match_token(&TokenType::While) {
            return self.parse_flow_control_fn();
        }
        else if self.match_token(&TokenType::PrintFn) {
            let fn_call_node =  self.parse_fn_calling("print".to_string(), true)?;
            return Ok(NodeStmt::FnCall(fn_call_node));
        }
        Err(format!("Unexpected {:?}, previous: {:?}, next {:?}", self.current_token(0), self.current_token(-1), self.current_token(1)))
    }
    

    fn parse_expr(&mut self) -> Result<NodeExpr, String> {
        if let Some(Token { kind: TokenType::IntLit(value), .. }) = self.current_token(0) {
            self.advance();
            if let Some(Token { kind: TokenType::Operators(_), .. }) = self.current_token(0) {
                return self.parse_math_expr(NodeExpr::IntLiteral(value));
            }
            return Ok(NodeExpr::IntLiteral(value));
        } 
        else if let Some(Token { kind: TokenType::Ident(ident), .. }) = self.current_token(0) {
            self.advance();
            if let Some(Token { kind: TokenType::Operators(_), .. }) = self.current_token(0) {
                return self.parse_math_expr(NodeExpr::Identifier(ident));
            }
            return Ok(NodeExpr::Identifier(ident));
        }
        Err(format!("Unexpected: {:?}", self.current_token(0)))
    }
    

    fn parse_math_expr(&mut self, left_side: NodeExpr) -> Result<NodeExpr, String> {
        if let Some(Token { kind: TokenType::Operators(operator), .. }) = self.current_token(0){
            self.advance(); // Consume the operator
            
            let right_expr = Box::new(self.parse_expr()?);
            let left_expr = Box::new(left_side);
            Ok(NodeExpr::MathOperat(NodeMathExpr {
                left_expr,
                operator,
                right_expr 
            }))
        }
        else {
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
    fn parse_flow_control_fn(&mut self) -> Result<NodeStmt, String> {
        let func = self.current_token(0).unwrap(); // Get 'if' || while etc.

        self.advance(); // Consume 'if || while || etc.'

        let condition = self.parse_equality()?;

        let scope = self.parse_scope()?;

        let enum_stmt: Option<BuiltInFunctions> = match func.kind {
            TokenType::If => {
                Some(BuiltInFunctions::If(Box::new(NodeIfStmt {
                    scope,
                    condition,
                })))
            }
            TokenType::While => {
                Some(BuiltInFunctions::While(Box::new(NodeWhileStmt {
                    scope,
                    condition,
                })))
            }
            _ => None
        };
        Ok(NodeStmt::CompilerBuiltInFunctions(enum_stmt.unwrap()))
    }

    fn parse_equality(&mut self) -> Result<NodeEquality, String> {
        let right_expr = self.parse_expr()?;
        let mut _is_inequality = false;

        if !self.match_token(&TokenType::Eq) && !self.match_token(&TokenType::ExclamationPoint) {
            return Err("Expected '==' or '!='".to_string());
        }

        if self.match_token(&TokenType::ExclamationPoint) {
            _is_inequality = true;
        }
        else {
            _is_inequality = false;
        }
        self.advance(); // Consume '=' || '!'

        if !self.match_token(&TokenType::Eq) {
            return Err("Expected '==' or '!='".to_string());
        }
        self.advance(); // Consume '='
        let left_expr = self.parse_expr()?;

        Ok(NodeEquality {
            right_expr,
            left_expr,
            is_inequality: _is_inequality,
        })
    }
    fn parse_fn_calling(&mut self, name: String, is_built_in: bool) -> Result<NodeFnCall, String> {
        self.advance(); // Consume fnname

        if !self.match_token(&TokenType::OpenParen) {
            return Err(format!("Expected '(' found {:?}, previous {:?}, next {:?}", self.current_token(0), self.current_token(-1), self.current_token(1)));
        }
        self.advance(); // Consume '('

        if let Some(Token { kind: TokenType::String(argument), .. }) = self.current_token(0)  {
            self.advance(); // Consume argument
            if !self.match_token(&TokenType::ClosedParen) {
                return Err(format!("Expected ')' found {:?}", self.current_token(0)));
            }
            self.advance(); // Consume ')'
    
            return Ok(NodeFnCall {
                name,
                is_built_in,
                argument
            });
        }
        return Err(format!("Expected argument String in print func! found {:?}", self.current_token(0)))        
    }
}