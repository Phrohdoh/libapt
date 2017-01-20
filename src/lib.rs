#[macro_use]
extern crate enum_primitive;

mod actionscript;

pub struct SwfHeader {
    pub is_compressed: bool,
    pub version: u8,
    pub len: u32,
    pub size_twips: (u32, u32),
    pub fps: u16,
    pub num_frames: u16,
}