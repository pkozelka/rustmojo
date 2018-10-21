#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::path::Path;
use serde_json::Value;

fn tocsv<P: AsRef<Path>>(path: P) {
    let file = File::open (path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
    match json {
        Value::Array(array) => for item in array {
            println!("item={}", item);
        },
        Value::Object(map) => for key in map.keys() {
            println!("key={} value={}", key, map.get(key).unwrap());
        },
        _ => println!("other")
    };
}

fn main() {
    let mojo = Mojo::load("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo").unwrap();
}
