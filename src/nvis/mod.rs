mod transformer;

use std::fmt;

use crate::nvis::transformer::*;

#[derive(Clone, PartialEq)]
pub enum InputMode {
    Raw,
    Smart,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputMode::Raw => write!(f, "Raw"),
            InputMode::Smart => write!(f, "Smart"),
        }
    }
}

pub struct Context {
    pub input_mode: InputMode,
    pub focus_idx: u16,
}

pub const TRANSFORMERS: [&'static Transformer; 16] = [
    &(Base64 {}),
    &(Base32 {}),
    &(Hex {}),
    &(CHex {}),
    &(LittleEndianU16 {}),
    &(BigEndianU16 {}),
    &(LittleEndianU32 {}),
    &(BigEndianU32 {}),
    &(LittleEndianU64 {}),
    &(BigEndianU64 {}),
    &(LittleEndianI16 {}),
    &(BigEndianI16 {}),
    &(LittleEndianI32 {}),
    &(BigEndianI32 {}),
    &(LittleEndianI64 {}),
    &(BigEndianI64 {}),
];

pub const NONE_PLACEHOLDER: &'static str = "<none>";
