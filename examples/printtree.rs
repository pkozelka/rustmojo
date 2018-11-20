extern crate rustmojo as this;

use this::mojoreader::MojoInformation;
use this::mojoreader::MojoReader;
use this::mojoreader::SplitValue;
use this::mojoreader::SubNode;
use std::fs::File;
use std::io::Read;
use std::io;

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
    let root = read_file(&mut file).expect("ERROR");
    match root {
        SubNode::Leaf(value) => println!("leaf value is {}", value),
        SubNode::NestedNode(_) => println!("subnode")
    }
    println!("byte count is {}", size);

    treeprint("", &root);

/*
    let mut position = 0;
    for byte in buf {
        println!("{:5} = 0x{0:04X} {:02X}", position, byte);
        position += 1;
    }
*/
}

fn read_file(file: &mut File) -> io::Result<SubNode> {
    let mut buf = Vec::new();
    let bytes = file.read_to_end(&mut buf)?;
    let mut reader = MojoReader::new(MojoInformation::new());
    reader.read_node(&mut buf.iter())
}
