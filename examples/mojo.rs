use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::slice::Iter;

enum SplitValue {
    IsNotANumber,
    IsLessOrEqualTo(f32),
    IsPresentInSet(u64/*TODO: &bitset!*/),
}

enum  SubNode {
    Leaf(f32),
    NestedNode(Box<MtrNode>)
}

struct MtrNode {
    address: u32,
    node_type: u8,
    split_dir: u8,
    split_column_id: u16,
    split_value: SplitValue,

    left: SubNode,

    right_node_address: u32,
    right:SubNode,
}

struct MojoInformation {
    mojo_version: u16,
    nclasses: u16,
    // columns, domains, ...
}

struct MojoReader {
    info: MojoInformation,
}

impl MojoReader {

    fn new(info: MojoInformation) -> MojoReader {
        MojoReader{info: info}
    }

    fn read_node(&mut self, input: &mut Iter<u8>) -> Result<SubNode, &Error> {
        let nodeflags = input.next().unwrap();
        println!("nodeflags {}", nodeflags);
        Ok(SubNode::Leaf(1.2))
    }
}

fn main() {
    println!("Hello");
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let mut buf = Vec::new();
    let bytes = file.read_to_end(&mut buf).unwrap();
    let mut reader = MojoReader::new(MojoInformation{mojo_version: 100, nclasses: 2});
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
