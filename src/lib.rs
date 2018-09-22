extern crate core;

mod rowdata;

use std::collections::HashMap;

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

#[cfg(test)]
mod tests;
