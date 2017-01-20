#[macro_use]
extern crate enum_primitive;

extern crate byteorder;
use byteorder::ReadBytesExt;

use std::io::{self, BufRead};

mod actionscript;

pub struct SwfHeader {
    pub is_compressed: bool,
    pub version: u8,
    pub len: u32,
    pub size_twips: (u32, u32),
    pub fps: u16,
    pub num_frames: u16,
}

#[derive(Debug)]
pub enum ReadError {
    StdIoError(io::Error),
    UnknownFormat(Vec<u8>),
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> ReadError {
        ReadError::StdIoError(e)
    }
}

pub fn parse_header(br: &mut BufRead) -> Result<SwfHeader, ReadError> {
    let sig1 = try!(br.read_u8()); // F => uncompressed, C => compressed
    let sig2 = try!(br.read_u8());
    let sig3 = try!(br.read_u8());

    match (sig1, sig2, sig3) {
        (b'F', b'W', b'S') |
        (b'C', b'W', b'S') => {}
        _ => return Err(ReadError::UnknownFormat(vec![sig1, sig2, sig3])),
    }

    let version = try!(br.read_u8());

    Ok(SwfHeader {
        is_compressed: sig1 == b'C',
        version: version,

        // TODO
        len: 0,
        size_twips: (0, 0),
        fps: 0,
        num_frames: 0,
    })
}