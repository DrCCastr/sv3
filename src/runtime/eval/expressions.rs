use crate::{ast::{BinaryExpr, Expr, Identifier, VarDeclaration}, environmment::Environmment, interpreter::evaluate, value::{EnumVariableType, SunVariable}};

pub fn eval_numeric_binary_expr(lhs: SunVariable, rhs: SunVariable, operator: String, env: Environmment) -> SunVariable {
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

pub fn eval_binary_expr(Binop: &BinaryExpr, env: Environmment) -> SunVariable {
    let left: &Box<dyn Expr> = &Binop.left;
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

pub fn eval_var_declaration(vardec: &VarDeclaration, mut env: Environmment) -> SunVariable {
    let mut value: SunVariable;
    if let Some(Value) = &vardec.value {
        env.declare_var(vardec.identifier.clone(), evaluate(Value.as_stmt(), env.clone()));
    }
    SunVariable::new()
}