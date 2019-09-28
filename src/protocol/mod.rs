pub mod buffer;
pub mod encoder;
pub mod decoder;
pub mod auth;

pub fn read_message<T: decoder::Decoder>(buffer: &mut buffer::Buffer) -> Result<Box<T>, decoder::DecodeErr> {
    match T::decode(buffer) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(e)
    }
}