use super::buffer::Buffer;

#[derive(Debug)]
pub enum DecodeErr {
    Err(String),
}

pub trait Decoder {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr>;
}
