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
    fn indent(&mut self) {
        for _i in 0..=self.scope {
            self.output.push_str("  ");
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
            NodeStmt::Scope(stmts) => self.visit_scope(stmts),
            NodeStmt::If(stmt) => self.visit_if(*stmt),
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
                    crate::lexer::tokens::Operator::Star => "*",
                    crate::lexer::tokens::Operator::FSlash => "/",
                };
                self.output.push_str(string_op);
                self.visit_expr(*math_expr.right_side);
            }
        }
    }
    fn visit_scope(&mut self, stmts: Vec<NodeStmt>) {
            self.indent();
            self.output.push_str("{ \n");

            self.indent();
            for stmt in stmts {
                self.visit_stmt(stmt);
                self.indent();
            }

            self.output.push_str("} \n");
    }
    fn visit_if(&mut self, ifstmt: NodeIfStmt) {
        self.output.push_str("if( ");

        self.visit_equality(ifstmt.condition);

        self.output.push_str(")");


        match ifstmt.scope {
            NodeStmt::Scope(scope) => {
                self.visit_scope(scope)
            }
            _ => {}
        }
    }
    fn visit_equality(&mut self, node_eq: NodeEquality) {
        self.visit_expr(node_eq.left_expr);
        self.output.push_str(" == ");
        self.visit_expr(node_eq.right_expr);
    }
}