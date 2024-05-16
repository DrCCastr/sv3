use crate::ast::{self, *};
use crate::runtime::value::*;
use crate::environmment::{self, *};
use std::fmt::format;
use std::ops::Deref;
use std::{any, vec};
use std::rc::Rc;

fn errorN(Char: &str) {
    println!("Stmt not reconized: {}", Char);
}

fn eval_program(program: &Program, env: Environmment) -> SunVariable {
    let mut lastEvaluated = SunVariable::new();
    
    for statement in &program.body {
        lastEvaluated = evaluate(&**statement, env.clone());
    }
    
    return lastEvaluated
}

fn eval_numeric_binary_expr(lhs: SunVariable, rhs: SunVariable, operator: String, env: Environmment) -> SunVariable {
    let mut result = 0.0;
    
    if operator == "+" {
        result = lhs.get_number() + rhs.get_number();
    } else if operator == "-" {
        result = lhs.get_number() - rhs.get_number();
    } else if operator == "*" {
        result = lhs.get_number() * rhs.get_number();
    } else if operator == "/" {
        result = lhs.get_number() / rhs.get_number();
    } else {
        result = lhs.get_number() % rhs.get_number();
    }
    
    return SunVariable::new().set_value(EnumVariableType::NUMBER, format!("{}", result));
}

fn eval_binary_expr(Binop: &BinaryExpr, env: Environmment) -> SunVariable {
    let left = &Binop.left;
    let right = &Binop.right;
    
    // Avaliando os lados esquerdo e direito da expressão binária
    let lhs = evaluate(left.as_stmt(), env.clone());
    let rhs = evaluate(right.as_stmt(), env.clone());
    
    if lhs.get_type() == &EnumVariableType::NUMBER && rhs.get_type() == &EnumVariableType::NUMBER {
        return eval_numeric_binary_expr(lhs, rhs, Binop.operator.clone(), env.clone());
    }
    
    return SunVariable::new().set_value(EnumVariableType::NIL, "");
}

pub fn eval_identifier(iden: &Identifier, env: Environmment) -> SunVariable {
    let val = env.look_up_var(iden.symbol.clone());
    return val;
}

pub fn evaluate(astNode: &dyn Stmt, env: Environmment) -> SunVariable {
    match astNode.get_kind() {
        NodeType::NumericLiteral => {
            if let Some(numeric_literal) = astNode.as_numeric_literal() {
                return SunVariable::new().set_value(EnumVariableType::NUMBER, format!("{}", numeric_literal.value));
            } else {
                errorN("NumericLiteral");
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::BinaryExpr => {
            if let Some(binary_expr) = astNode.as_binary_expr() {
                return eval_binary_expr(binary_expr, env);
            } else {
                errorN("BinaryExpr");
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::Identifier => {
            if let Some(identifier) = astNode.as_identifier() {
                return eval_identifier(identifier, env);
            } else {
                errorN("Identifier");
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::Program => {
            if let Some(program) = astNode.as_program() {
                return  eval_program(program, env);
            } else {
                errorN("Program");
                return  SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        _ => {
            errorN("Default");
            return SunVariable::new().set_value(EnumVariableType::NIL, "");
        }
    }
}