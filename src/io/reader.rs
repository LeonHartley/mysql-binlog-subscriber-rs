use std::net::TcpStream;
use std::io::{Write};
use crate::protocol::buffer::{Buffer, reader::BufferReader, writer::BufferWriter};
use crate::protocol::decoder::{Decoder, DecodeErr};

pub fn read_message<T: Decoder>(buffer: &mut Buffer) -> Result<Box<T>, DecodeErr> {
    let length = match buffer.read_i32(3) {
        Ok(n) => n,
        Err(e) => return Err(DecodeErr::Err(format!("failed to decode length, {:?}", e)))
    };

    let n = buffer.read_u8();

    println!("got length={}", length);

    match T::decode(buffer) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(e)
    }
}