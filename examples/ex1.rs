extern crate chrono;
extern crate rustmojo;

use rustmojo::MojoModel;
use chrono::Local;

fn abspath(relpath: &str) -> Result<String,&str> {
    match std::env::home_dir() {
        Some(pathbuf) =>
            match pathbuf.to_str() {
                None => Err(""),
                Some(p) => Ok(String::from(format!("{}/{}", p, relpath)))
            },
        None => Err("")
    }
}

fn main() {
    println!("Example1");
    let filename = abspath("h2o/h2o-mojo/example/prostate").unwrap();
    let model = MojoModel::load(filename.as_str()).unwrap();
    println!("model: {}", model.nfeatures);
    let f: f32 = "3.21".parse().unwrap();
    let g = "0.1".parse::<f64>().unwrap();
    println!("f = {} g={}", f,g);
    let date = Local::now();
    println!("{}", date.format("%Y-%m-%dT%H:%M:%S"));
}
