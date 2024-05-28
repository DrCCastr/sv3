use core::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariableType {
    NIL, 
    NUMBER,
    STRING, 
    BOOLEAN, 
    FUNCTION, 
    OBJECT
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    Object_data: HashMap<String, SunVariable>,
}

#[derive(PartialEq, Clone)]
pub struct SunVariable {
    type_: EnumVariableType,
    string_value: String,
    number_value: f64,
    bool_value: bool,
    Object_value: Object
}

impl Object {
    pub fn new() -> Self {
        Self {
            Object_data: HashMap::new()
        }
    }
    pub fn add_variable(&mut self, name: String, variable: SunVariable) {
        self.Object_data.insert(name, variable);
    }
    
    pub fn remove_variable(&mut self, name: &str) {
        self.Object_data.remove(name);
    }
}

impl fmt::Debug for SunVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.type_ {
            EnumVariableType::NIL => {
                write!(f, "SunVariable [nil]")
            }
            EnumVariableType::NUMBER => {
                write!(f, "SunVariable Number: {}", self.number_value)
            }
            EnumVariableType::STRING => {
                write!(f, "SunVariable String: {}", self.string_value)
            }
            EnumVariableType::BOOLEAN => {
                write!(f, "SunVariable Boolean: {}", self.bool_value)
            }
            EnumVariableType::OBJECT => {
                write!(f, "SunVariable Object: {:#?}", self.Object_value)
            }
            _ => {
                write!(f, "SunVariable [Invalid]")
            }
        }
    }
}

impl SunVariable {
    pub fn new() -> Self {
        Self {
            type_: EnumVariableType::NIL,
            string_value: String::new(),
            number_value: 0.0,
            bool_value: false,
            Object_value: Object::new(),
        }
    }
    pub fn set_value(&mut self, new_type: EnumVariableType, value: impl Into<String>) -> SunVariable {
        match new_type {
            EnumVariableType::NUMBER => {
                if let Ok(num) = value.into().parse::<f64>() {
                    self.number_value = num;
                }
            },
            EnumVariableType::STRING => {
                self.string_value = value.into();
            },
            EnumVariableType::BOOLEAN => {
                if let Ok(b) = value.into().parse::<bool>() {
                    self.bool_value = b;
                }
            },
            _ => {}
        }
        self.type_ = new_type;
        return self.clone();
    }

    pub fn get_type(&self) -> &EnumVariableType {
        &self.type_
    }

    pub fn get_string(&self) -> &str {
        &self.string_value
    }

    pub fn get_number(&self) -> f64 {
        self.number_value
    }

    pub fn get_bool(&self) -> bool {
        self.bool_value
    }
    
    pub fn get_object(&mut self) -> &mut Object {
        &mut self.Object_value
    }
}