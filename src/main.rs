extern crate rustmojo;

use rustmojo::mojo::Mojo;
use std::collections::HashMap;

fn main() {
    let mojo = Mojo::load("data/gbm_v1.00_names.mojo").unwrap();
    let row: HashMap<&str,&str> = [
        ("AGE", "68"),
        ("RACE", "2"),
        ("DCAPS", "2"),
    ].iter().cloned().collect();

    println!("--------------");
    let prediction = mojo.predict_binomial(row).unwrap();
    println!("--------------");
    println!("Prediction: index={}, label='{}'", prediction.labelIndex, prediction.label);
    println!("P0: {}", prediction.p0);
    println!("P1: {}", prediction.p1);
}
