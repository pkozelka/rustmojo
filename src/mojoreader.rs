extern crate std;

use std::io::Error;
use std::io::ErrorKind;
use std::slice::Iter;

use acqua::acquamodel::*;
use mojoflags::MojoFlags;
use mojoflags::SplitValueType;
use std::fs::File;
use std::io;
use std::io::Read;

enum NaSplitDir {
    None,
    NAvsREST,
    NALeft,
    NARight,
    Left,
    Right,
}

pub struct MojoInformation {
    pub mojo_version: u16,
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

struct ByteArrayReader<'a>  {
    input: &'a mut Iter<'a, u8>
}

impl <'a> ByteArrayReader<'a> {

    fn new(input: &'a mut Iter<'a, u8>) -> ByteArrayReader<'a> {
        ByteArrayReader {
            input
        }
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        match self.input.next() {
            None => Err(Error::new(ErrorKind::UnexpectedEof, "oh no")),
            Some(&byte) => {
                println!(".... {:02X}", byte);
                Ok(byte)
            }
        }
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let l: u16 = self.read_u8()? as u16;
        let h: u16 = self.read_u8()? as u16;
        Ok((h << 8) + l)
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        let l: u32 = self.read_u16()? as u32;
        let h: u32 = self.read_u16()? as u32;
        Ok((h << 16) + l)
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        let value = self.read_u32()?;
        let num: f32 = unsafe { std::mem::transmute(value) };
        Ok(num)
    }

    fn skip(&mut self, nbytes: u16) -> Result<(), Error> {
        for _ in 0..nbytes {
            self.read_u8()?;
        }
        Ok(())
    }

    fn read_direction(&mut self) -> Result<NaSplitDir, Error> {
        let dirbyte = self.read_u8()?;
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
}

fn bits_to_bytes(nbits: u32) -> u32 {
    ((nbits-1) >> 3) + 1
}

impl MojoReader {

    pub fn new(info: MojoInformation) -> MojoReader {
        MojoReader{info: info}
    }

    pub fn read_tree_from_file(&self, file: &mut File) -> io::Result<Node> {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let input = &mut buf.iter();
        let ba = &mut ByteArrayReader::new(input);
        self.read_tree(ba)
    }

    fn read_tree(&self, ba: &mut ByteArrayReader) -> Result<Node, Error> {

        let flagbyte = ba.read_u8()?;
        let nodeflags = MojoFlags::new(flagbyte)?;
        println!("nodeflags[{:02X}]: left is leaf: {}, right is leaf: {}, offset_size = {}",
                 flagbyte,
                 nodeflags.left_node_is_leaf,
                 nodeflags.right_node_is_leaf,
                 nodeflags.offset_size);
        let split_column_id = ba.read_u16()?;
        println!("field_no {}", split_column_id);

        if split_column_id == 0xFFFF {
            return Ok(Node::ValueNode(ba.read_f32()?))
        }

        let dir = ba.read_direction()?;
//        println!("direction: {:?}", dir);

        let condition: Condition;

        if let NaSplitDir::NAvsREST = dir {
            condition = Condition {
                nan: NoNumberHandling::AsTrue,
                comparison: Comparison::None,
                invert: false
            };
        } else {
            let leftward = match dir {
                NaSplitDir::Left | NaSplitDir::NALeft => true,
                _ => false,
            };
            condition = match nodeflags.split_value_type {
                SplitValueType::Number => Condition {
                    nan: if leftward {NoNumberHandling::AsFalse} else {NoNumberHandling::AsTrue},
                    comparison: Comparison::Numeric(ba.read_f32()?),
                    invert: false
                },
                SplitValueType::Bitset => {
                    let bit_offset = ba.read_u16()?;
                    if self.info.mojo_version < 130 {
                        let nbytes = ba.read_u16()?;
                        println!("bitset[{},{}]", bit_offset, nbytes);
                        ba.skip(nbytes)?;
                    } else {
                        let nbits = ba.read_u32()?;
                        let nbytes = bits_to_bytes(nbits);
                        println!("bitset[{},{}]", bit_offset, nbytes);
                        ba.skip(nbytes as u16)?;
                    }
                    println!("--");
                    Condition {
                        nan: NoNumberHandling::None,
                        comparison: Comparison::Bitset(Box::new(MojoBitset::new(/*todo*/))),
                        invert: false
                    }
                },
                SplitValueType::Bitset32 => {
                    let _bits = ba.read_u32()?;
                    Condition {
                        nan: if leftward {NoNumberHandling::AsFalse} else {NoNumberHandling::AsTrue},
                        comparison: Comparison::Bitset(Box::new(MojoBitset::new(/*todo*/))),
                        invert: false
                    }
                },
            };
        };

        let left_node = if nodeflags.left_node_is_leaf {
            let leaf = ba.read_f32()?;
            println!("left leaf: {}", leaf);
            Node::ValueNode(leaf)
        } else {
            println!("offset");
            ba.skip(nodeflags.offset_size as u16)?;
            println!("left node");
            self.read_tree(ba)?
        };

        let right_node = if nodeflags.right_node_is_leaf {
            let leaf = ba.read_f32()?;
            println!("right leaf: {}", leaf);
            Node::ValueNode(leaf)
        } else {
            println!("right node");
            self.read_tree(ba)?
        };

        Ok(Node::DecisionNode(DecisionNode{
            column: Box::new(Col::new(split_column_id)),
            condition,
            do_then: Box::new(left_node),
            do_else: Box::new(right_node)
        }))
    }
}

pub struct MojoBitset {}

impl MojoBitset {
    fn new() -> MojoBitset {
        MojoBitset{}
    }
}

impl Bitset for MojoBitset {
    fn get(&self, _bit: i32) -> bool {
        unimplemented!()
    }
}
