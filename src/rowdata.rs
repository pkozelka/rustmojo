use std::fmt::Display;
use std::fmt::Formatter;
use core::fmt;
use std::collections::HashMap;

pub enum ValueType {
    None,
    Text(String),
    Number(f64),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ValueType::None => write!(f, "None"),
            ValueType::Text(s) => write!(f, "string('{}')", s),
            ValueType::Number(n) => write!(f, "number({})", n),
        }
    }
}

pub struct RowData {
    data: HashMap<String,ValueType>
}

impl RowData {
    pub fn new() -> RowData {
        return RowData {
            data: HashMap::new()
        }
    }

    pub fn set(& mut self, key: &str, value: ValueType) {
        self.data.insert(String::from(key), value);
    }

    pub fn get(&self, key: &str) -> &ValueType {
        match self.data.get(key) {
            Option::None => &ValueType::None,
            Option::Some(x) => x
        }
    }
}
