use crate::ast::*;
use crate::runtime::value::*;
use std::fmt::format;
use std::ops::Deref;
use std::vec;
use std::rc::Rc;

fn eval_program(program: &Program) -> SunVariable {
    let mut lastEvaluated = SunVariable::new();
    
    for statement in &program.body {
        lastEvaluated = evaluate(&**statement);
    }
    
    lastEvaluated
}

fn eval_binary_expr(Binop: &BinaryExpr) -> SunVariable {
    let left = &Binop.left;
    let right = &Binop.right;
    
    // Avaliando os lados esquerdo e direito da expressão binária
    let lhs = evaluate(left.as_stmt());
    let rhs = evaluate(right.as_stmt());
    
    if (lhs.get_type(), rhs.get_type()) == (&EnumVariableType::NUMBER, &EnumVariableType::NUMBER) {
        
    }
    
    return SunVariable::new().set_value(EnumVariableType::NIL, "");
}

pub fn evaluate(astNode: &dyn Stmt) -> SunVariable {
    match astNode.get_kind() {
        NodeType::NumericLiteral => {
            if let Some(numeric_literal) = astNode.as_numeric_literal() {
                return SunVariable::new().set_value(EnumVariableType::NUMBER, format!("{}", numeric_literal.value));
            } else {
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::BinaryExpr => {
            if let Some(binary_expr) = astNode.as_binary_expr() {
                return eval_binary_expr(binary_expr);
            } else {
                return SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        NodeType::Program => {
            if let Some(program) = astNode.as_program() {
                return  eval_program(program);
            } else {
                return  SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        _ => {
            return SunVariable::new().set_value(EnumVariableType::NIL, "");
        }
    }
}