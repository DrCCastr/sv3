use std::collections::HashMap;
use crate::value::*;

#[derive(Clone, PartialEq)]
pub struct Environmment<'a> {
    parent: Option<&'a Environmment<'a>>,
    variables: HashMap<String, SunVariable>
}

impl<'a> Environmment<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new()
        }
    }

    pub fn set_parent(&mut self, parent_env: &'a Environmment<'a>) {
        self.parent = Some(parent_env);
    }

    pub fn declare_var(&mut self, var_name: String, value: SunVariable) -> SunVariable {
        if !self.variables.contains_key(&var_name) {
            let newvalue = value.clone();
            self.variables.insert(var_name, newvalue);
            return value;
        } else {
            println!("Cannot declare variable {}. At is already is defined", var_name);
        }
        
        return value;
    }

    pub fn look_up_var(mut self, var_name: String) -> SunVariable {
        let mut env = self.resolve(var_name.clone());
        if let Some(var) = env.variables.get(&var_name) {
            return var.clone();
        }
        
        return SunVariable::new();
    }

    pub fn assign_var(mut self, var_name: String, value: SunVariable) -> SunVariable {
        let mut env = self.resolve(var_name.clone());
        env.variables.remove(&var_name.clone());
        env.variables.insert(var_name, value.clone());
        
        return value;
    }

    pub fn resolve(self, var_name: String) -> Environmment<'a> {
        if self.variables.contains_key(&var_name) {
            return self;
        } else if self.parent == None {
            println!("Cannot resolve {} as it does not exist", var_name);
            return Environmment::new();
        }
        
        if let Some(parent) = self.parent {
            return parent.clone().resolve(var_name);
        } else {
            return self.resolve(var_name);
        }
    }
}