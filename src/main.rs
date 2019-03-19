extern crate rustmojo;

use std::collections::HashMap;

use rustmojo::mojo::Mojo;

fn main() {
    let mojo = Mojo::load("data/gbm_v1.00_names.mojo").unwrap();
//    let mojo = Mojo::load("/home/pk/h2o/h2o-mojo/testmodels/prostate/unzipped").unwrap();
    let row: HashMap<&str,&str> = [
        ("AGE", "68"),
        ("RACE", "2"),
        ("DCAPS", "2"),
    ].iter().cloned().collect();

    println!("--------------");
    let prediction = mojo.predict_binomial_easy(row).unwrap();
    println!("--------------");
    println!("Prediction: index={}, label='{}'", prediction.label_index, prediction.label);
    let vec: Vec<String> = prediction.pred.iter().map(|n| format!("{}", n)).collect();
    println!("predictions: {}", vec.join(","));
}
