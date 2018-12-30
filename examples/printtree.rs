extern crate rustmojo as this;

use std::fs::File;
use std::io;
use std::io::Read;

use this::acqua::acquamodel::Comparison;
use this::acqua::acquamodel::Node;
use this::mojoreader::MojoInformation;
use this::mojoreader::MojoReader;

fn treeprint(indent: &str, node: &Node) {
    match node {
        Node::ValueNode(value) => {
            println!("{}   {}", indent, value)
        },
        Node::DecisionNode(decision) => {
            let comparison = match decision.condition.comparison {
                Comparison::None => {
                    String::from("true")
                },
                Comparison::IsLessThan(f) => {
                    format!("is < {}", f)
                },
                Comparison::BitsetContains(_) => {
                    String::from("is in set(...todo...)")
                },
            };
            println!("{}   if Col{} {}", indent, decision.column.get_column_no(), comparison);
            let mut indent = String::from(indent.clone());
            indent.push(' ');
            indent.push(' ');
            println!("{} then", indent);
            treeprint(&indent, &decision.do_then);
            println!("{} else", indent);
            treeprint(&indent, &decision.do_else);
        },
    }
}

fn main() {
    println!("Hello");
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let root = read_file(&mut file).expect("ERROR");
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

fn read_file(file: &mut File) -> io::Result<Node> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let mut reader = MojoReader::new(MojoInformation::new());
    reader.read_tree(&mut buf.iter())
}
