use std::collections::HashMap;

struct DomainInfo {
    num_levels: u32, // TODO: is this same as levels.size??
    filename: String,
    levels: Vec<String>,
}

struct MojoModelInfo {
    properties: HashMap<String, String>,
    columns: Vec<String>,
    domains: HashMap<i32, DomainInfo>,
    domain_lengths: Vec<i32>,
}

enum ModelCategory {
    Unknown      = 0,
    Binomial     = 1,
    Multinomial  = 2,
    Regression   = 3,
    Clustering   = 4,
    AutoEncoder  = 5,
    DimReduction = 6
}

struct MojoModel {
    mojo_version_major: i32,
    mojo_version_minor: i32,
    info: MojoModelInfo,
    category: ModelCategory,
    uuid: String,
    supervised: bool,
    nfeatures: i32,
    nclasses: i32,
    balance_classes: i32,
    default_threshold: i32,
    prior_class_distrib: Vec<f32>,
    model_class_distrib: Vec<f32>,
    empty_vector_of_strings: Vec<String>,
}

impl MojoModel {
    fn load(filename: &str) -> Result<MojoModel, &str> {
        panic!("not implemented yet");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use MojoModel;

    #[test]
    fn can_load() {
        let mm = MojoModel::load("test.mdl")?;
//        let ModelCategory c = ModelCategory::Binomial();
    }
}
