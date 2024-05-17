use crate::{ast::Program, environmment::Environmment, interpreter::evaluate, value::SunVariable};

pub fn eval_program(program: &Program, env: Environmment) -> SunVariable {
    let mut lastEvaluated = SunVariable::new();
    
    for statement in &program.body {
        lastEvaluated = evaluate(&**statement, env.clone());
    }
    
    return lastEvaluated
}