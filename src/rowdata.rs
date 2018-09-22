use std::fmt::Display;
use std::fmt::Formatter;
use core::fmt;
use std::collections::HashMap;

pub enum ValueType {
    Text(String),
    Number(f64),
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ValueType::Text(s) => write!(f, "string('{}')", s),
            ValueType::Number(n) => write!(f, "number({})", n),
        }
    }
}

pub type RowData = HashMap<String, ValueType>;

