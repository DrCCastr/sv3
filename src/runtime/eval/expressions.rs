use crate::{ast::{AssignmentExpr, BinaryExpr, Expr, Identifier, NodeType, ObjectLiteral, Stmt, VarDeclaration}, environmment::{self, Environmment}, interpreter::evaluate, runtime, value::{EnumVariableType, Object, SunVariable}};

pub fn eval_numeric_binary_expr(lhs: SunVariable, rhs: SunVariable, operator: String, env: &Environmment) -> SunVariable {
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

pub fn eval_binary_expr(Binop: &BinaryExpr, env: &mut Environmment) -> SunVariable {
    let left: &Box<dyn Expr> = &Binop.left;
    let right = &Binop.right;
    
    // Avaliando os lados esquerdo e direito da expressão binária
    let lhs = evaluate(left.as_stmt(), env);
    let rhs = evaluate(right.as_stmt(), env);
    
    if lhs.get_type() == &EnumVariableType::NUMBER && rhs.get_type() == &EnumVariableType::NUMBER {
        return eval_numeric_binary_expr(lhs, rhs, Binop.operator.clone(), env);
    }
    
    return SunVariable::new().set_value(EnumVariableType::NIL, "");
}

pub fn eval_identifier(iden: &Identifier, env: &mut Environmment) -> SunVariable {
    let val = env.clone().look_up_var(iden.symbol.clone());
    return val;
}

pub fn eval_object_expr(obj: &ObjectLiteral, env: &mut Environmment) -> SunVariable {
    let mut object = SunVariable::new().set_value(EnumVariableType::OBJECT, "");
    
    for value in &obj.value {
        if let Some(val) = value.1.value.as_ref() {
            let mut runtime_val = if value.1.value.is_none() {
                env.clone().look_up_var(value.0.to_string())
            } else {
                evaluate(val.as_stmt(), env)
            };
            
            object.get_object().add_variable(value.0.to_string(), runtime_val);
        }
    }
    
    object
}


pub fn eval_var_declaration(vardec: &VarDeclaration, env: &mut Environmment) -> SunVariable {
    if let Some(Value) = &vardec.value {
        let value = evaluate(Value.as_stmt(), env);
        env.declare_var(vardec.identifier.clone(), value, vardec.constant, false);
    }
    SunVariable::new()
}

pub fn eval_assingment(node: &AssignmentExpr, env: &mut Environmment) -> SunVariable {
    if node.assgine.get_kind() != NodeType::Identifier {
        println!("Invalide LHS inaide assingment Expr {:#?}", node.assgine);
        std::process::exit(1);
    }
    let mut varname = String::new();
    varname = "j".to_string();
    if let Some(iden) = node.as_identifier() {
        varname = iden.symbol.clone();
    }
    let value = evaluate(node.value.as_stmt(), &mut env.clone());
    
    return env.declare_var(varname, value, true, true);
}