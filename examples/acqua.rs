extern crate rustmojo as this;

use std::fs::File;

use this::mojoreader::MojoInformation;
use this::mojoreader::MojoReader;

fn main() {
    let mojo_reader = MojoReader::new(MojoInformation::new());
    let a = mojo_reader.read_tree_from_file(&mut File::open("data/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap());
    if let Err(e) = a {
        eprintln!("ERROR: {}", e.to_string())
    }
}
