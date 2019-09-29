pub mod buffer;
pub mod encoder;
pub mod decoder;
pub mod auth;

use std::net::TcpStream;
use std::io::{Read, Write};
use buffer::{Buffer, reader::BufferReader, writer::BufferWriter};

pub fn read_message<T: decoder::Decoder>(buffer: &mut buffer::Buffer) -> Result<Box<T>, decoder::DecodeErr> {
    let length = match buffer.read_i32(3) {
        Ok(n) => n,
        Err(e) => return Err(decoder::DecodeErr::Err(format!("failed to decode length, {:?}", e)))
    };

    let n = buffer.read_u8();

    println!("got length={}", length);

    match T::decode(buffer) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(e)
    }
}

pub fn write_message<T: encoder::Encoder>(msg: &mut T, stream: &mut TcpStream) {
    let mut message = match msg.encode() {
        Ok(msg) => msg,
        Err(_) => return
    };

    let length = message.length();
    let message_bytes = message.read_bytes(length);
    println!("sending message length={}", length);

    match message_bytes {
        Ok(bytes) => {
            let mut buffer = Buffer::empty()
                .write_i32(length as i32, 3)
                .write_bytes(&bytes);

            stream.write(buffer.into_bytes());
        },
        Err(e) => println!("failed to send buffer, {:?}", e)
    };
}

#[cfg(test)]
pub mod test;
