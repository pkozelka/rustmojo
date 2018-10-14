use std::fs::File;
use std::io::Read;

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

fn read_node() -> MtrNode {
    MtrNode {
        address: 0,
        node_type: 0,
        split_dir: 0,
        split_column_id: 0,
        split_value: SplitValue::IsLessOrEqualTo(0.0),
        left: SubNode::Leaf(1.2),
        right_node_address: 0,
        right: SubNode::Leaf(3.4),
    }
}

fn main() {
    println!("Hello");
    let mut file= File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap();
    let size = file.metadata().unwrap().len();
    println!("file size is {}", size);
    let mut buf = Vec::new();
    let bytes = file.read_to_end(&mut buf).unwrap();
    println!("byte count is {}", bytes);

    let mut position = 0;
    for byte in buf {
        println!("{:5} = 0x{0:04X} {:02X}", position, byte);
        position += 1;
    }
}
