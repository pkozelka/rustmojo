extern crate rustmojo;

use rustmojo::mojoreader::MojoInformation;
use rustmojo::mojoreader::MojoReader;
use rustmojo::mojoreader::SplitValue;
use rustmojo::mojoreader::SubNode;
use std::fs::File;
use std::io::Read;

fn treeprint(indent: &str, node: &SubNode) {
    match node {
        SubNode::Leaf(value) => {
            println!("{}   {}", indent, value)
        },
        SubNode::NestedNode(split) => {
            let condition = match split.split_value {
                SplitValue::IsNotANumber => {
                    String::from("is NaN ?")
                },
                SplitValue::IsLessOrEqualTo(f) => {
                    format!("is <= {}", f)
                },
                SplitValue::IsPresentInSet(_) => {
                    String::from("is in set(...todo...)")
                },
            };
            println!("{}   split: Col{} is {}", indent, split.split_column_id, condition);
            let mut indent = String::from(indent.clone());
            indent.push(' ');
            indent.push(' ');
            println!("{} left:", indent);
            treeprint(&indent, &split.left);
            println!("{} right:", indent);
            treeprint(&indent, &split.right);
        },
    }
}

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

    treeprint("", &root);

/*
    let mut position = 0;
    for byte in buf {
        println!("{:5} = 0x{0:04X} {:02X}", position, byte);
        position += 1;
    }
*/
}
