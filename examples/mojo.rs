extern crate rustmojo;

use rustmojo::mojoreader::MojoInformation;
use rustmojo::mojoreader::MojoReader;
use rustmojo::mojoreader::SubNode;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello");
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let mut buf = Vec::new();
    let bytes = file.read_to_end(&mut buf).unwrap();
    let mut reader = MojoReader::new(MojoInformation::new());
    let root = reader.read_node(&mut buf.iter()).expect("ERROR");
    match root {
        SubNode::Leaf(value) => println!("leaf value is {}", value),
        SubNode::NestedNode(_) => println!("subnode")
    }
    println!("byte count is {}", bytes);

/*
    let mut position = 0;
    for byte in buf {
        println!("{:5} = 0x{0:04X} {:02X}", position, byte);
        position += 1;
    }
*/
}
