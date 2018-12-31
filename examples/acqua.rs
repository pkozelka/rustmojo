extern crate rustmojo as this;

use std::fs::File;

use this::mojoreader::MojoReader;
use this::mojoreader::MojoInformation;

fn main() {
    let mojo_reader = MojoReader::new(MojoInformation::new());
    let a = mojo_reader::read_tree_from_file(&mut File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap());
    if let Err(e) = a {
        eprintln!("ERROR: {}", e.to_string())
    }
}
