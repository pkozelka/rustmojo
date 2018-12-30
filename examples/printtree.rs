extern crate rustmojo as this;

use std::fs::File;
use std::io;
use std::io::Read;

use this::acqua::acquamodel::Comparison;
use this::acqua::acquamodel::Node;
use this::mojoreader::MojoInformation;
use this::mojoreader::MojoReader;

fn treeprint(node: &Node) {
    println!("Tree:");
    treeprint_level(0, node);
}

fn print_indent(i: usize) {
    for _ in 0..i {
        print!("  ");
    }
}

fn treeprint_level(indent: usize, node: &Node) {
    print_indent(indent);
    match node {
        Node::ValueNode(value) => {
            println!("{}", value)
        },
        Node::DecisionNode(decision) => {
            let mut ifline = format!("Col{}", decision.column.get_column_no());
            match decision.condition.comparison {
                Comparison::None => {
                    ifline.push_str(" :true");
                },
                Comparison::IsLessThan(f) => {
                    ifline.push_str(&format!(" < {}", f));
                },
                Comparison::BitsetContains(_) => {
                    ifline.push_str(" is in set(...todo...)");
                },
            };
            if decision.condition.is_na {
                ifline.push_str(" || isNA")
            }
            if decision.condition.invert {
                ifline.insert_str(0, "!(");
                ifline.push_str(")");
            }
            println!("if {}", ifline);
            print_indent(indent);
            println!("then");
            treeprint_level(indent + 1, &decision.do_then);
            print_indent(indent);
            println!("else");
            treeprint_level(indent + 1, &decision.do_else);
        },
    }
}

fn read_file(file: &mut File) -> io::Result<Node> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let mut reader = MojoReader::new(MojoInformation::new());
    reader.read_tree(&mut buf.iter())
}

fn main() {
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let root = read_file(&mut file).expect("ERROR");
    println!("byte count is {}", size);

    treeprint(&root);
}
