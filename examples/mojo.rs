extern crate rustmojo;

use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;
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

fn read_u8(input: &mut Iter<u8>) -> Result<u8, Error> {
    match input.next() {
        None => Err(Error::new(ErrorKind::UnexpectedEof, "oh no")),
        Some(byte) => Ok(*byte)
    }
}

fn read_u16(input: &mut Iter<u8>) -> Result<u16, Error> {
    let l: u16 = read_u8(input)? as u16;
    let h: u16 = read_u8(input)? as u16;
    Ok((h << 8) + l)
}

fn read_u32(input: &mut Iter<u8>) -> Result<u32, Error> {
    let l: u32 = read_u8(input)? as u32;
    let h: u32 = read_u8(input)? as u32;
    Ok((h << 16) + l)
}

fn read_f32(input: &mut Iter<u8>) -> Result<f32, Error> {
    let value = read_u32(input)?;
    let num: f32 = unsafe { std::mem::transmute(value)};
    Ok(num)
}

impl MojoReader {

    fn new(info: MojoInformation) -> MojoReader {
        MojoReader{info: info}
    }

    fn read_node(&mut self, input: &mut Iter<u8>) -> Result<SubNode, Error> {
        let nodeflags = rustmojo::mojoflags::MojoFlags::new(*input.next().unwrap())?;
        println!("nodeflags.offset_size = {}", nodeflags.offset_size);
        let field_no = read_u16(input)?;
        println!("field_no {}", field_no);
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
