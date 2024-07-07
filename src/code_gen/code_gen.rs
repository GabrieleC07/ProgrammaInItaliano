use std::collections::HashMap;

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
            NodeStmt::VarDecl(name, var_decl_possible) => self.visit_var_decl(name, var_decl_possible),
            NodeStmt::VarShadowing(name, expr) => self.visit_var_shadowing(name, expr),
            NodeStmt::Scope(stmts) => self.visit_scope(stmts),
            NodeStmt::CompilerBuiltInFunctions(func) => {
                match func {
                    BuiltInFunctions::If(ifstmt) => { 
                        self.visit_flow_control(BuiltInFunctions::If(ifstmt), "if");
                    }
                    BuiltInFunctions::While(whilestmt) => {
                            self.visit_flow_control(BuiltInFunctions::While(whilestmt), "while");
                        }
                    }
                }
                NodeStmt::FnCall(fn_call_node) => self.visit_fn_call(fn_call_node),
            }
        }

    fn visit_return(&mut self, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str("return ");
        self.visit_expr(expr);
        self.output.push_str(";\n");
    }

    fn visit_var_decl(&mut self, name: String, var_types: VarDeclTypes) {
        self.output.push_str("    "); 
        match var_types {
            VarDeclTypes::Expr(expr) => {
                self.output.push_str("int ");
                self.output.push_str(&name);
                self.output.push_str(" = ");
                self.visit_expr(expr);
                self.output.push_str(";\n");
            }
            VarDeclTypes::String(string) => {
                self.output.push_str("char ");
                self.output.push_str(&format!("{}[]", &name));
                self.output.push_str(" = ");
                self.output.push_str(&string);
                self.output.push_str(";\n");
            },
            VarDeclTypes::Bool(bool) => {
                self.output.push_str("bool ");
                self.output.push_str(&name);
                self.output.push_str(" = ");
                self.output.push_str(&bool.to_string());
                self.output.push_str(";\n");
            }
        }
        
    }
    fn visit_var_shadowing(&mut self, name: String, expr: NodeExpr) {
        self.output.push_str("    ");
        self.output.push_str(&name);
        self.output.push_str(" = ");
        self.visit_expr(expr);
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
                self.visit_expr(*math_expr.left_expr);
                let string_op = match math_expr.operator {
                    crate::lexer::tokens::Operator::Plus => "+",
                    crate::lexer::tokens::Operator::Minus => "-",
                    crate::lexer::tokens::Operator::Star => "*",
                    crate::lexer::tokens::Operator::FSlash => "/",
                };
                self.output.push_str(string_op);
                self.visit_expr(*math_expr.right_expr);
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
    fn visit_flow_control(&mut self, func_enum: BuiltInFunctions, kind: &str) {
        self.output.push_str(&format!("{}(", kind));

        let condition = match func_enum {
            BuiltInFunctions::If(ref if_stmt) => &if_stmt.condition,
            BuiltInFunctions::While(ref while_stmt) => &while_stmt.condition,
        };

        self.visit_equality(condition.clone());

        self.output.push_str(")");

        
        match func_enum {
            BuiltInFunctions::If(if_stmt) => {
                match if_stmt.scope {
                    NodeStmt::Scope(stmts) => self.visit_scope(stmts),
                    _ => {}
                }
            }
            BuiltInFunctions::While(while_stmt) => {
                match while_stmt.scope {
                    NodeStmt::Scope(stmts) => self.visit_scope(stmts),
                    _ => {}
                }
            }
        }
    }
    fn visit_equality(&mut self, node_eq: NodeEquality) {
        self.visit_expr(node_eq.left_expr);

        if node_eq.is_inequality {
            self.output.push_str(" != ");
        }
        else {
            self.output.push_str(" == ");
        }

        self.visit_expr(node_eq.right_expr);
    }
    fn visit_fn_call(&mut self, node_fn_call: NodeFnCall) {
        let map = self.create_built_in_func_map();

        if node_fn_call.is_built_in {
            let optional_c_name = map.get(&node_fn_call.name);
            if optional_c_name.is_none() {
                panic!("EXPECTED A BUILT-IN FUNCTION! GOT THIS {}", node_fn_call.name);
            }
            self.output.push_str(&format!("{}({}\n);\n", optional_c_name.unwrap(), node_fn_call.argument))
        }
        else {
            self.output.push_str(&format!("{}();", node_fn_call.name))
        }
    }
    fn create_built_in_func_map(&mut self) -> HashMap<String, String> {
        let mut map = HashMap::new();
    
        map.insert("print".to_string(), "printf".to_string());
        
        map
    }
}