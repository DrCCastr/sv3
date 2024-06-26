use std::collections::HashMap;
use std::vec::Vec;
use crate::value::{self, *};

#[derive(Clone, PartialEq)]
pub struct Environmment<'a> {
    parent: Option<&'a Environmment<'a>>,
    variables: HashMap<String, SunVariable>,
    constants: Vec<String>
}

pub fn createGlobalEnv() -> Environmment<'static> {
    let mut env = Environmment::new(None);
    env.declare_var("true".to_string(), SunVariable::new().set_value(value::EnumVariableType::BOOLEAN, "true"), true, false);
    env.declare_var("false".to_string(), SunVariable::new().set_value(value::EnumVariableType::BOOLEAN, "false"), true, false);
    env.declare_var("nil".to_string(), SunVariable::new().set_value(value::EnumVariableType::NIL, ""), true, false);
    return env;
}

impl<'a> Environmment<'a> {
    pub fn new(parent: Option<&'a Environmment<'a>>) -> Self {
        let global = parent.is_some();
        let mut env = Self {
            parent: parent,
            variables: HashMap::new(),
            constants: Vec::new()
        };
        if global {
            env = createGlobalEnv()
        }
        
        env
    }

    pub fn set_parent(&mut self, parent_env: &'a Environmment<'a>) {
        self.parent = Some(parent_env);
    }

    pub fn declare_var(&mut self, var_name: String, value: SunVariable, constant: bool, assingining: bool) -> SunVariable {
        if !self.variables.contains_key(&var_name) {
            let newvalue = value.clone();
            self.variables.insert(var_name.clone(), newvalue);
            if constant == true {
                self.constants.insert(0, var_name);
            }
            return value;
        } else {
            if assingining {
                self.variables.remove(&var_name.clone());
                return self.declare_var(var_name, value, constant, true);
            }
            println!("Cannot declare variable {}. At is already is defined", var_name);
            std::process::exit(1);
        }
    }

    pub fn look_up_var(mut self, var_name: String) -> SunVariable {
        let mut env = self.resolve(var_name.clone());
        if let Some(var) = env.variables.get(&var_name) {
            return var.clone();
        }
        
        return SunVariable::new();
    }

    /*pub fn assign_var(&mut self, var_name: String, value: SunVariable) -> SunVariable {
        let mut env = self.clone().resolve(var_name.clone());
        if self.constants.contains(&var_name) {
            println!("cannot can change the {} value, is a constant", var_name);
            std::process::exit(1);
        }
        env.variables.remove(&var_name.clone());
        env.variables.insert(var_name.clone(), value.clone());
        
        return value;
    }*/

    pub fn resolve(self, var_name: String) -> Environmment<'a> {
        if self.variables.contains_key(&var_name) {
            return self;
        } else if self.parent == None {
            println!("Cannot resolve {} as it does not exist", var_name);
            std::process::exit(1);
        }
        
        if let Some(parent) = self.parent {
            return parent.clone().resolve(var_name);
        } else {
            return self.resolve(var_name);
        }
    }
}