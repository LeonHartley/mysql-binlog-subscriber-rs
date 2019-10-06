use crate::protocol::buffer::{Buffer};
use crate::protocol::decoder::{Decoder, DecodeErr};

pub fn read_message<T: Decoder>(buffer: &mut Buffer) -> Result<Box<T>, DecodeErr> {
    match T::decode(buffer) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(e)
    }
}

pub fn read_generic_message<T: Decoder>(buffer: &mut Buffer) -> Result<Box<T>, DecodeErr> {
    match T::decode(buffer) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(e)
    }
}