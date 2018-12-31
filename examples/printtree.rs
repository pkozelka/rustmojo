extern crate rustmojo as this;

use std::fs::File;

use this::acqua::acquamodel::Comparison;
use this::acqua::acquamodel::Node;
use this::mojoreader::MojoReader;
use this::acqua::acquamodel::NoNumberHandling;
use this::mojoreader::MojoInformation;

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
            let col_name = format!("Col{}", decision.column.get_column_no());
            let mut ifline = String::new();
            match decision.condition.nan {
                NoNumberHandling::None => {},
                NoNumberHandling::AsTrue => ifline.push_str(&format!("{}.isNAN() || ", col_name)),
                NoNumberHandling::AsFalse => ifline.push_str(&format!("!{}.isNAN() && ", col_name)),
            }
            match decision.condition.comparison {
                Comparison::None => {},
                Comparison::IsLessThan(f) => {
                    ifline.push_str(&col_name);
                    if decision.condition.invert {
                        ifline.push_str(&format!(" >= {}", f));
                    } else {
                        ifline.push_str(&format!(" < {}", f));
                    }
                },
                Comparison::BitsetContains(_) => {
                    if decision.condition.invert {
                        ifline.push('!');
                    }
                    ifline.push_str(&format!("set(...).contains({})", col_name));
                },
            };
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

fn main() {
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let mojo_reader = MojoReader::new(MojoInformation::new());
    let root = mojo_reader.read_tree_from_file(&mut file).expect("ERROR");
    println!("byte count is {}", size);

    treeprint(&root);
}
