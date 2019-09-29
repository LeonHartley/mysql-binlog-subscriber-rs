use super::buffer::reader::BufferReader;

#[derive(Debug)]
pub enum DecodeErr {
    Err(String),
}

pub trait Decoder {
    fn decode(buffer: &mut BufferReader) -> Result<Box<Self>, DecodeErr>;
}
