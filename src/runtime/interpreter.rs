use crate::ast::{self, *};
use crate::runtime::value::*;
use crate::environmment::{self, *};
use crate::eval::*;
use std::fmt::format;
use std::ops::Deref;
use std::{any, vec};
use std::rc::Rc;

fn errorN(Char: &str) {
    println!("Stmt not reconized: {}", Char);
    std::process::exit(1);
}

pub fn evaluate(astNode: &dyn Stmt, env: &mut Environmment) -> SunVariable {
    match astNode.get_kind() {
        NodeType::NumericLiteral => {
            if let Some(numeric_literal) = astNode.as_numeric_literal() {
                return SunVariable::new().set_value(EnumVariableType::NUMBER, format!("{}", numeric_literal.value));
            } else {
                errorN(&format!("{:#?}", astNode).to_string());
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::BinaryExpr => {
            if let Some(binary_expr) = astNode.as_binary_expr() {
                return expressions::eval_binary_expr(binary_expr, env);
            } else {
                errorN(&format!("{:#?}", astNode).to_string());
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::Identifier => {
            if let Some(identifier) = astNode.as_identifier() {
                return expressions::eval_identifier(identifier, env);
            } else {
                errorN(&format!("{:#?}", astNode).to_string());
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::VarDeclarationStmt => {
            if let Some(vardeclaration) = astNode.as_var_declaration() {
                return expressions::eval_var_declaration(&vardeclaration, env);
            } else {
                errorN(&format!("{:#?}", astNode).to_string());
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::AssignmentExpr => {
            if let Some(asn) = astNode.as_assignment_expr() {
                return expressions::eval_assingment(asn, env);
            } else {
                errorN("abc");
                return SunVariable::new();
            }
        }
        NodeType::Program => {
            if let Some(program) = astNode.as_program() {
                return statements::eval_program(program, env);
            } else {
                errorN(&format!("{:#?}", astNode).to_string());
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        _ => {
            println!("{:#?}", astNode);
            errorN(&format!("{:#?}", astNode).to_string());
            return SunVariable::new().set_value(EnumVariableType::NIL, "");
        }
    }
}