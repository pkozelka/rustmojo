use std::fmt::Display;
use std::fmt::Formatter;
use core::fmt;

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

pub trait RowData {
    fn set(&self, key: &str, value: ValueType);
    fn get(&self, key: &str) -> ValueType;
}

pub struct RowDataStruct {
    pub xvalue: f64
}

impl RowData for RowDataStruct {
    fn set(&self, key: &str, value: ValueType) {
        println!("ok");
    }

    fn get(&self, key: &str) -> ValueType {
        ValueType::Number(self.xvalue)
    }
}

