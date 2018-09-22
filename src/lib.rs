extern crate core;

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::error::Error;
use core::fmt;

pub struct DomainInfo {
    num_levels: u32, // TODO: is this same as levels.size??
    filename: String,
    levels: Vec<String>,
}

pub struct MojoModelInfo {
    properties: HashMap<String, String>,
    columns: Vec<String>,
    domains: HashMap<i32, DomainInfo>,
    domain_lengths: Vec<i32>,
}

/// A model category.
pub enum ModelCategory {
    /// we don't know
    Unknown      = 0,

    /** simply true or false */
    Binomial     = 1,
    Multinomial  = 2,
    Regression   = 3,
    Clustering   = 4,
    AutoEncoder  = 5,
    DimReduction = 6
}

/**
 * some mojo model
 */
pub struct MojoModel {
    mojo_version_major: i32,
    mojo_version_minor: i32,
    info: MojoModelInfo,
    category: ModelCategory,
    uuid: String,
    supervised: bool,
    pub nfeatures: i32,
    nclasses: i32,
    balance_classes: i32,
    default_threshold: i32,
    prior_class_distrib: Vec<f32>,
    model_class_distrib: Vec<f32>,
    empty_vector_of_strings: Vec<String>,
}

impl MojoModel {
    pub fn load(filename: &str) -> Result<MojoModel, &str> {
        println!("Loading {}", &filename);
        return Ok(MojoModel {
            mojo_version_major: 0,
            mojo_version_minor: 0,
            info: MojoModelInfo {
                properties: HashMap::new(),
                columns: Vec::new(),
                domains: HashMap::new(),
                domain_lengths: Vec::new(),
            },
            uuid: String::new(),
            supervised: false,
            nfeatures: 0,
            nclasses: 0,
            balance_classes: 0,
            default_threshold: 0,
            prior_class_distrib: Vec::new(),
            model_class_distrib: Vec::new(),
            empty_vector_of_strings: Vec::new(),
            category: ModelCategory::Binomial,
        });
//        panic!("not implemented yet");
    }
}

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
    xvalue: f64
}

impl RowData for RowDataStruct {
    fn set(&self, key: &str, value: ValueType) {
        println!("ok");
    }

    fn get(&self, key: &str) -> ValueType {
        ValueType::Number(self.xvalue)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use MojoModel;
    use ::ModelCategory;

    #[test]
    fn can_load() {
        let mm = MojoModel::load("test.mdl");
        let a = ModelCategory::Regression;
        let c = ModelCategory::Binomial;
    }


    #[test]
    fn test_rowdata() {
        let data = ::RowDataStruct{
            xvalue: 1.2346,
        };

        use RowData;

        data.set("a",::ValueType::Text(String::from("Hello")));

        println!("value = {}", data.get("haha"));
    }
}
