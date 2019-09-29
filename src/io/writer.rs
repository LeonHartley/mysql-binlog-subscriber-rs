use std::net::TcpStream;
use std::io::{Write};
use crate::protocol::buffer::{Buffer, reader::BufferReader, writer::BufferWriter};
use crate::protocol::encoder::{Encoder, EncodeErr};

pub fn write_message<T: Encoder>(msg: &mut T, stream: &mut TcpStream) {
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