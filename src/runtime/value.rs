use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariableType {
    NIL, 
    NUMBER,
    STRING, 
    BOOLEAN, 
    FUNCTION, 
    TABLE
}

#[derive(Debug, PartialEq, Clone)]
pub struct Table {
    table_data: HashMap<String, SunVariable>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SunVariable {
    type_: EnumVariableType,
    string_value: String,
    number_value: f64,
    bool_value: bool,
    // function_value: Option<fn()>,
    table_value: Table, // Utiliza a estrutura Table do módulo table
}

impl Table {
    pub fn new() -> Self {
        Self {
            table_data: HashMap::new()
        }
    }
    pub fn add_variable(&mut self, name: String, variable: SunVariable) {
        self.table_data.insert(name, variable);
    }
    
    pub fn remove_variable(&mut self, name: &str) {
        self.table_data.remove(name);
    }
}

impl SunVariable {
    pub fn new() -> Self {
        Self {
            type_: EnumVariableType::NIL,
            string_value: String::new(),
            number_value: 0.0,
            bool_value: false,
            table_value: Table::new(),
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

    pub fn get_table(&self) -> &Table {
        &self.table_value
    }
}