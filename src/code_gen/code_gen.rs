use crate::parser::nodes::*;

pub struct CodeGenerator {
    output: String,
    scope: u8,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            scope: 0,
        }
    }

    pub fn generate(&mut self, stmts: Vec<NodeStmt>) -> String {
        self.output.push_str("#include <stdio.h>\n\n");
        self.output.push_str("int main() {\n");
        for stmt in stmts {
            self.visit_stmt(stmt);
        }
        self.output.push_str("}\n");
        self.output.clone()
    }

    fn visit_stmt(&mut self, stmt: NodeStmt) {
        match stmt {
            NodeStmt::Return(expr) => self.visit_return(expr),
            NodeStmt::VarDecl(name, expr) => self.visit_var_decl(name, expr),
            NodeStmt::Scope(stms) => { 
                for stmt in stms {
                    
                    for i in 0..=self.scope {
                        self.output.push_str("  ");
                    }
                    self.output.push_str("{ \n");

                    for i in 0..=self.scope {
                        self.output.push_str("  ");
                    }

                    self.visit_stmt(stmt);

                    for i in 0..=self.scope {
                        self.output.push_str("  ");
                    }
                    self.output.push_str("} \n");
                }
            }
        }
    }

    fn visit_return(&mut self, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str("return ");
        self.visit_expr(expr);
        self.output.push_str(";\n");
    }

    fn visit_var_decl(&mut self, name: String, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str("int ");
        self.output.push_str(&name);
        self.output.push_str(" = ");
        self.visit_expr(expr);
        self.output.push_str(";\n");
    }

    fn visit_expr(&mut self, expr: NodeExpr) {
        match expr {
            NodeExpr::IntLiteral(value) => {
                self.output.push_str(&value.to_string());
            }
            NodeExpr::Identifier(name) => {
                self.output.push_str(&name);
            }
            NodeExpr::MathOperat(math_expr) => {
                self.visit_expr(*math_expr.left_side);
                let string_op = match math_expr.operator {
                    crate::lexer::tokens::Operator::Plus => "+",
                    crate::lexer::tokens::Operator::Minus => "-",
                    crate::lexer::tokens::Operator::Mul => "*",
                    crate::lexer::tokens::Operator::Div => "/",
                };
                self.output.push_str(string_op);
                self.visit_expr(*math_expr.right_side);
            }
        }
    }
    fn visit_end_of_file(&mut self) {

    }
}