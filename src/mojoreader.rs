extern crate std;

use mojoflags::MojoFlags;
use std::io::Error;
use std::io::ErrorKind;
use std::slice::Iter;
use mojoflags::SplitValueType;

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

fn skip(input: &mut Iter<u8>, nbytes: u16) -> Result<(), Error> {
    for i in 0..nbytes {
        read_u8(input)?;
    }
    Ok(())
}

fn read_direction(input: &mut Iter<u8>) -> Result<NaSplitDir, Error>{
    match read_u8(input)? {
        0 => Ok(NaSplitDir::None),
        1 => Ok(NaSplitDir::NAvsREST),
        2 => Ok(NaSplitDir::NALeft),
        3 => Ok(NaSplitDir::NARight),
        4 => Ok(NaSplitDir::Left),
        5 => Ok(NaSplitDir::Right),
        _ => Err(Error::new(ErrorKind::InvalidData, "Invalid direction"))
    }
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

        let dir = read_direction(input)?;
//        println!("direction: {}", dir);

        let split_value: SplitValue;

        if let NaSplitDir::NAvsREST = dir {
            split_value = SplitValue::IsNotANumber;
        } else {
            match nodeflags.split_value_type {
                SplitValueType::Number => {
                    split_value = SplitValue::IsLessOrEqualTo(read_f32(input)?);
                },
                SplitValueType::Bitset => {
                    let bit_offset = read_u16(input)?;
                    let nbytes = read_u16(input)?;
                    split_value = SplitValue::IsPresentInSet(0 /*todo!*/);
                    skip(input, nbytes);
                },
                SplitValueType::Bitset32 => {
                    let bits = read_u32(input)?;
                    split_value = SplitValue::IsPresentInSet(bits /*todo!*/);
                },
            }
        }


        Ok(SubNode::Leaf(1.2))
    }
}
