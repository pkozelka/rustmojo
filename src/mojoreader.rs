extern crate std;

use mojoflags::MojoFlags;
use mojoflags::SplitValueType;
use std::io::Error;
use std::io::ErrorKind;
use std::slice::Iter;

pub enum SplitValue {
    IsNotANumber,
    IsLessOrEqualTo(f32),
    IsPresentInSet(u32 /*TODO: &bitset!*/),
}

enum NaSplitDir {
    None,
    NAvsREST,
    NALeft,
    NARight,
    Left,
    Right,
}

pub enum SubNode {
    Leaf(f32),
    NestedNode(Box<MtrNode>)
}

pub struct MtrNode {
//    address: u32,
//    node_type: u8,
//    split_dir: u8,
//    split_column_id: u16,
    pub split_value: SplitValue,

    left: SubNode,

//    right_node_address: u32,
    right:SubNode,
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
        Some(&byte) => {
            println!(".... {:02X}", byte);
            Ok(byte)
        }
    }
}

fn read_u16(input: &mut Iter<u8>) -> Result<u16, Error> {
    let l: u16 = read_u8(input)? as u16;
    let h: u16 = read_u8(input)? as u16;
    Ok((h << 8) + l)
}

fn read_u32(input: &mut Iter<u8>) -> Result<u32, Error> {
    let l: u32 = read_u16(input)? as u32;
    let h: u32 = read_u16(input)? as u32;
    Ok((h << 16) + l)
}

fn read_f32(input: &mut Iter<u8>) -> Result<f32, Error> {
    let value = read_u32(input)?;
    let num: f32 = unsafe { std::mem::transmute(value)};
    Ok(num)
}

fn skip(input: &mut Iter<u8>, nbytes: u16) -> Result<(), Error> {   
    for _ in 0..nbytes {
        read_u8(input)?;
    }
    Ok(())
}

fn read_direction(input: &mut Iter<u8>) -> Result<NaSplitDir, Error>{
    let dirbyte = read_u8(input)?;
    println!("dirbyte: {}", dirbyte);
    match dirbyte {
        0 => Ok(NaSplitDir::None),
        1 => Ok(NaSplitDir::NAvsREST),
        2 => Ok(NaSplitDir::NALeft),
        3 => Ok(NaSplitDir::NARight),
        4 => Ok(NaSplitDir::Left),
        5 => Ok(NaSplitDir::Right),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid direction"))
    }
}

fn bits_to_bytes(nbits: u32) -> u32 {
    ((nbits-1) >> 3) + 1
}

impl MojoReader {

    pub fn new(info: MojoInformation) -> MojoReader {
        MojoReader{info: info}
    }

    pub fn read_node(&mut self, input: &mut Iter<u8>) -> Result<SubNode, Error> {
        let flagbyte = read_u8(input)?;
        let nodeflags = MojoFlags::new(flagbyte)?;
        println!("nodeflags[{:02X}]: left is leaf: {}, right is leaf: {}, offset_size = {}",
                 flagbyte,
                 nodeflags.left_node_is_leaf,
                 nodeflags.right_node_is_leaf,
                 nodeflags.offset_size);
        let field_no = read_u16(input)?;
        println!("field_no {}", field_no);

        if field_no == 0xFFFF {
            return Ok(SubNode::Leaf(read_f32(input)?))
        }

        let dir = read_direction(input)?;
//        println!("direction: {}", dir);

        let split_value: SplitValue;

        if let NaSplitDir::NAvsREST = dir {
            split_value = SplitValue::IsNotANumber;
        } else {
            split_value = match nodeflags.split_value_type {
                SplitValueType::Number => {
                    SplitValue::IsLessOrEqualTo(read_f32(input)?)
                },
                SplitValueType::Bitset => {
                    let bit_offset = read_u16(input)?;
                    if self.info.mojo_version < 130 {
                        let nbytes = read_u16(input)?;
                        println!("bitset[{},{}]", bit_offset, nbytes);
                        skip(input, nbytes)?;
                    } else {
                        let nbits = read_u32(input)?;
                        let nbytes = bits_to_bytes(nbits);
                        println!("bitset[{},{}]", bit_offset, nbytes);
                        skip(input, nbytes as u16)?;
                    }
                    println!("--");
                    SplitValue::IsPresentInSet(0 /*todo!*/)
                },
                SplitValueType::Bitset32 => {
                    let bits = read_u32(input)?;
                    SplitValue::IsPresentInSet(bits /*todo!*/)
                },
            };
        };

        let left_node = if nodeflags.left_node_is_leaf {
            let leaf = read_f32(input)?;
            println!("left leaf: {}", leaf);
            SubNode::Leaf(leaf)
        } else {
            println!("offset");
            skip(input, nodeflags.offset_size as u16)?;
            println!("left node");
            self.read_node(input)?
        };

        let right_node = if nodeflags.right_node_is_leaf {
            let leaf = read_f32(input)?;
            println!("right leaf: {}", leaf);
            SubNode::Leaf(leaf)
        } else {
            println!("right node");
            self.read_node(input)?
        };

        let node = MtrNode{
            split_value: (split_value),
            left: (left_node),
            right: (right_node),
        };
        Ok(SubNode::NestedNode(Box::new(node)))
    }
}
