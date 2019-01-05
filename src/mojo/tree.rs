use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use mojoreader::MojoInformation;
use mojoreader::MojoReader;
use std::fs::File;
use acqua::acquamodel::Node;
use acqua::acquamodel::NoNumberHandling;
use acqua::acquamodel::Comparison;
use mojo::modelini;

pub struct Mojo {
    trees: Vec<Vec<Node>>
}

pub struct BinomialPrediction {
    pub label_index: u32,
    pub label: String,
    pub pred: Vec<f64>,
}

impl Mojo {
    pub fn load<P: AsRef<Path>>(p: P) -> io::Result<Mojo> {
        if p.as_ref().is_file() {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Reading zipped mojos is not yet implemented"));
        }
        println!("HELLO FROM Mojo::load('{}')", p.as_ref().to_path_buf().into_os_string().to_str().unwrap());
        let model_ini_path = &p;
        println!("modelini: '{}'", model_ini_path.as_ref().to_str().unwrap());
        let modelini = modelini::ModelIni::parse(model_ini_path)?;

        for (key,value) in modelini.s_info {
            println!("info['{}']='{}'", key, value);
        }

        println!("-*-*-*-");
        println!("mojo_version: {}", modelini.info.mojo_version);
        println!("n_classes: {}", modelini.info.n_classes);
        println!("n_trees: {}", modelini.info.n_trees);
        println!("n_trees_per_class: {}", modelini.info.n_trees_per_class);
        println!("n_domains: {}", modelini.info.n_domains);
        println!("default_threshold: {}", modelini.info.default_threshold);
        println!("-*-*-*-");
        println!("\nColumns: {}", modelini.columns.join(","));
        println!("\nDomains:");
        for (name,domain) in modelini.domains {
            println!("/{}/ = [{:03}] {} file:'{}' levels:{} {{{}}}", name, domain.col_index,
                     domain.col_name,
                     domain.file_name,
                     domain.values.len(),
                     domain.values.join(","),
            );
        }
        // read all trees
        let mojo_reader = MojoReader::new(MojoInformation {
            mojo_version: modelini.info.mojo_version as u16
        });
        let groups= if modelini.info.n_classes > 2 { modelini.info.n_classes } else { 1 };
        let mut trees = Vec::new();
        for i in 0..groups {
            let mut class_trees = Vec::new();
            for j in 0..modelini.info.n_trees {
                let res = format!("trees/t{:02}_{:03}.bin", i, j);
                println!("{} / {}", p.as_ref().to_str().unwrap_or("???"), res);
                let tree_path = p.as_ref().join(res).canonicalize()?;
                println!("loading tree from {}", tree_path.to_str().unwrap_or("???"));
                let mut tree_file = File::open(tree_path)?;
                let tree = mojo_reader.read_tree_from_file(&mut tree_file)?; // todo
                class_trees.push(tree);
            }
            trees.push(class_trees);
        }
        Ok(Mojo{trees})
    }

    fn gbm_score(&self, node: &Node, row: &Vec<f64>) -> io::Result<f64> {
        match node {
            Node::ValueNode(rv) => Ok(*rv),
            Node::DecisionNode(decision) => {
                let index = decision.column.get_column_no();
                let value = *row.get(index).ok_or(Error::new(ErrorKind::InvalidData, format!("invalid column index: {}", index)))?;
                let test_nan = if value.is_nan() {
                    match decision.condition.nan {
                        NoNumberHandling::None => return Err(Error::new(ErrorKind::InvalidData, "NAN occurred where not expected")),
                        NoNumberHandling::AsTrue => true,
                        NoNumberHandling::AsFalse => false,
                    }
                } else {
                    false
                };

                let goright = test_nan && match &decision.condition.comparison {
                    Comparison::None => true,
                    Comparison::Numeric(split_val) => value >= *split_val,
                    Comparison::Bitset(_bs) => false, // TODO
                };
                let rnode = if goright {
                    &decision.do_then
                } else {
                    &decision.do_else
                };
                self.gbm_score(rnode, row)
            }
        }
    }

    pub fn gbm_predict(&self, row: &Vec<f64>) -> io::Result<BinomialPrediction>{
//        let _preds = gbm::score(&row, 0f64)?;
        // GBM prediction

        let mut pred = Vec::new();
        for group in &self.trees {
            let mut group_pred = 0.0;
            for tree in group {
                let p = self.gbm_score(&tree, row)?;
                group_pred += p;
            }
            pred.push(group_pred);
        }
        if self.trees.len() == 1 {
            let complement = 1.0 - &pred[0];
            &pred.push(complement);
        }

        Ok(BinomialPrediction{
            label_index: 0,
            label: String::from("RAW_DUMMY"),
            pred,
        })
    }

    pub fn predict_binomial_easy(&self, easy_row: HashMap<&str, &str>) -> io::Result<BinomialPrediction>{
        let mut row = Vec::<f64>::new();
        for (_key, value) in easy_row {
            match value.parse::<f64>() {
                Ok(v) => row.push(v),
                Err(_) => return Err(Error::new(ErrorKind::InvalidData, format!("cannot parse '{}' float", value))),
            }
        }
        // TODO: convert row from hashmap to vector of floats
        /*
        TODO:
        - preamble:
            - EasyPredictModelWrapper::validateModelCategory
            - predict
                - EasyPredictModelWrapper::fillRawData: convert row to vector of doubles
                - preds = GbmMojoModel::score0(raw)
                    - GbmMojoModel::scoreAllTrees
                        - clear preds (no cummulation)
                        - for i in trees_per_group
                            - for j in tree_in_group(i)
                                - preds[k] + SharedTreeMojoModel::scoreTreeV(tree, raw)
                    - switch by distributionFamily:
                        - bernoulli, modified_huber
                        - multinomial
                    - if MojoModel::balanceClasses then MojoModel::correctProbabilities
                    - MojoModel::getPrediction(preds, row, ...)
                - compute second value (1-first)
        */
        self.gbm_predict(&row)
    }
}
