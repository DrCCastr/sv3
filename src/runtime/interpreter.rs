use crate::ast::{*};
use crate::runtime::value::*;
use std::fmt::format;
use std::vec;

pub fn evaluate(astNode: &dyn Stmt) -> SunVariable {
    match astNode.get_kind() {
        NodeType::NumericLiteral => {
            if let Some(numeric_literal) = astNode.as_numeric_literal() {
                return SunVariable::new().set_value(EnumVariableType::NUMBER, format!("{}", numeric_literal.value));
            } else {
                return  SunVariable::new().set_value(EnumVariableType::NIL, "");
            }
        }
        _ => {
            return SunVariable::new().set_value(EnumVariableType::NIL, "");
        }
    }
}