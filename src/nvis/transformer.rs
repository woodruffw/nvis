use std::fmt::Write;
use std::io::Cursor;

use base32;
use base64;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use hex;

/* TODO(ww): byteorder could be replaced with uXX.from_{le,be}_bytes.
 */

pub trait Transformer {
    fn label(&self) -> &'static str;
    fn transform(&self, input: &[u8]) -> String;
}

pub struct Base64;
impl Transformer for Base64 {
    fn label(&self) -> &'static str {
        "base64"
    }

    fn transform(&self, input: &[u8]) -> String {
        let output = base64::encode_config(input, base64::STANDARD_NO_PAD);

        if output.is_empty() {
            return String::from(crate::nvis::NONE_PLACEHOLDER);
        } else {
            return output;
        }
    }
}

pub struct Base32;
impl Transformer for Base32 {
    fn label(&self) -> &'static str {
        "base32"
    }

    fn transform(&self, input: &[u8]) -> String {
        let output = base32::encode(base32::Alphabet::RFC4648 { padding: false }, input);

        if output.is_empty() {
            return String::from(crate::nvis::NONE_PLACEHOLDER);
        } else {
            return output;
        }
    }
}

pub struct Hex;
impl Transformer for Hex {
    fn label(&self) -> &'static str {
        "hex"
    }

    fn transform(&self, input: &[u8]) -> String {
        let output = hex::encode(input);

        if output.is_empty() {
            return String::from(crate::nvis::NONE_PLACEHOLDER);
        } else {
            return output;
        }
    }
}

pub struct CHex;
impl Transformer for CHex {
    fn label(&self) -> &'static str {
        "chex"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut buf = String::new();

        for &byte in input {
            write!(&mut buf, "\\x{:02X}", byte).expect("Couldn't format string");
        }

        if buf.is_empty() {
            return String::from(crate::nvis::NONE_PLACEHOLDER);
        } else {
            return buf;
        }
    }
}

pub struct LittleEndianU16;
impl Transformer for LittleEndianU16 {
    fn label(&self) -> &'static str {
        "leu16"
    }

    fn transform(&self, input: &[u8]) -> String {
        if input.len() != 2 {
            return String::from(crate::nvis::NONE_PLACEHOLDER);
        }

        let mut rdr = Cursor::new(input);

        match rdr.read_u16::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianU16;
impl Transformer for BigEndianU16 {
    fn label(&self) -> &'static str {
        "beu16"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_u16::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct LittleEndianU32;
impl Transformer for LittleEndianU32 {
    fn label(&self) -> &'static str {
        "leu32"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_u32::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianU32;
impl Transformer for BigEndianU32 {
    fn label(&self) -> &'static str {
        "beu32"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_u32::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct LittleEndianU64;
impl Transformer for LittleEndianU64 {
    fn label(&self) -> &'static str {
        "leu64"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_u64::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianU64;
impl Transformer for BigEndianU64 {
    fn label(&self) -> &'static str {
        "beu64"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_u64::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct LittleEndianI16;
impl Transformer for LittleEndianI16 {
    fn label(&self) -> &'static str {
        "lei16"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i16::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianI16;
impl Transformer for BigEndianI16 {
    fn label(&self) -> &'static str {
        "bei16"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i16::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct LittleEndianI32;
impl Transformer for LittleEndianI32 {
    fn label(&self) -> &'static str {
        "lei32"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i32::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianI32;
impl Transformer for BigEndianI32 {
    fn label(&self) -> &'static str {
        "bei32"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i32::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct LittleEndianI64;
impl Transformer for LittleEndianI64 {
    fn label(&self) -> &'static str {
        "lei64"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i64::<LittleEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}

pub struct BigEndianI64;
impl Transformer for BigEndianI64 {
    fn label(&self) -> &'static str {
        "bei64"
    }

    fn transform(&self, input: &[u8]) -> String {
        let mut rdr = Cursor::new(input);

        match rdr.read_i64::<BigEndian>() {
            Ok(n) => return n.to_string(),
            Err(_e) => return String::from(crate::nvis::NONE_PLACEHOLDER),
        }
    }
}
