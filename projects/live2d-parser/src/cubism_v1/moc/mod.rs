mod params;
mod parts;

use self::parts::Part;
use crate::cubism_v1::moc::params::Parameter;
use integer_encoding::VarInt;
use serde::de::Error;
use std::str;

pub struct Moc<'i> {
    /// The version of the moc file
    version: u8,
    /// Parameter list
    params: Vec<Parameter<'i>>,
    /// Parts list
    parts: Vec<Part>,
    /// Canvas width
    canvas_width: i32,
    /// Canvas height
    canvas_height: i32,
}

enum ObjectType {
    Unknown,
}

impl<'i> Moc<'i> {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &'i [u8]) -> Result<Self, serde_json::Error> {
        // Parse parameters and parts
        let (params, rest) = Parameter::parse_many(&data)?;
        let (parts, rest) = Part::parse_many(&rest)?;
        Ok(Self { version: 0, params, parts, canvas_width: 0, canvas_height: 0 })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the parameter list
    pub fn parameters(&self) -> &[Parameter] {
        &[]
    }

    /// Get the parts list
    pub fn parts(&self) -> &[Part] {
        &self.parts
    }

    /// Get the canvas width
    pub fn canvas_width(&self) -> i32 {
        self.canvas_width
    }

    /// Get the canvas height
    pub fn canvas_height(&self) -> i32 {
        self.canvas_height
    }
}

unsafe fn read_str(bytes: &[u8]) -> Result<(&str, &[u8]), serde_json::Error> {
    let (length, delta) = match u64::decode_var(bytes) {
        Some(s) => s,
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    };
    let end = delta + length as usize;
    let str = std::str::from_utf8_unchecked(bytes.get_unchecked(delta..end));
    let rest = bytes.get_unchecked(end..);
    Ok((str, rest))
}
