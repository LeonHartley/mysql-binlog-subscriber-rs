use super::buffer::Buffer;

pub enum EncodeErr {
    Err(String)
}

pub trait Encoder {
    fn encode(&mut self) -> Result<Buffer, EncodeErr>;
}