use std::io::Error;
use std::io::ErrorKind;

pub enum SplitValueType {
    Number,
    Bitset,
    Bitset32,
}

pub struct MojoFlags {
    flags: u8,
    pub left_node_is_leaf: bool,
    pub right_node_is_leaf: bool,
    pub offset_size: u8,
    pub split_value_type: SplitValueType,
}

impl MojoFlags {

    pub fn new(flags: u8) -> Result<MojoFlags, Error> {
        let left_node_is_leaf = match flags & 0x30 {
            0x00 => false,
            0x30 => true,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid lmask value"))
        };
        let right_node_is_leaf = match flags & 0xC0 {
            0x00 => false,
            0xC0 => true,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid rmask value"))
        };
        let split_value_type = match flags & 0x0C {
            0x00 => SplitValueType::Number,
            0x08 => SplitValueType::Bitset32,
            0x0C => SplitValueType::Bitset,
            _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid split_value_type value"))
        };
        Ok(MojoFlags{
            flags: flags,
            left_node_is_leaf: left_node_is_leaf,
            right_node_is_leaf: right_node_is_leaf,
            offset_size: (flags & 0x03) + 1,
            split_value_type: split_value_type,
        })
    }

}
