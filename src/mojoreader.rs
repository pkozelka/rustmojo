extern crate std;

use mojoflags::MojoFlags;
use std::io::Error;
use std::io::ErrorKind;
use std::slice::Iter;

/*
enum SplitValue {
    IsNotANumber,
    IsLessOrEqualTo(f32),
    IsPresentInSet(u64*//*TODO: &bitset!*//*),
}
*/

pub enum SubNode {
    Leaf(f32),
    NestedNode(Box<MtrNode>)
}

pub struct MtrNode {
//    address: u32,
//    node_type: u8,
//    split_dir: u8,
//    split_column_id: u16,
//    split_value: SplitValue,

//    left: SubNode,

//    right_node_address: u32,
//    right:SubNode,
}

pub struct MojoInformation {
    mojo_version: u16,
    // columns, domains, ...
}

impl MojoInformation {
    pub fn new() -> MojoInformation {
        MojoInformation{mojo_version: 100}
    }
}

pub struct MojoReader {
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

    pub fn new(info: MojoInformation) -> MojoReader {
        MojoReader{info: info}
    }

    pub fn read_node(&mut self, input: &mut Iter<u8>) -> Result<SubNode, Error> {
        let nodeflags = MojoFlags::new(*input.next().unwrap())?;
        println!("nodeflags.offset_size = {}", nodeflags.offset_size);
        let field_no = read_u16(input)?;
        println!("field_no {}", field_no);

        if field_no == 0xFFFF {
            return Ok(SubNode::Leaf(read_f32(input)?))
        }
        Ok(SubNode::Leaf(1.2))
    }
}
