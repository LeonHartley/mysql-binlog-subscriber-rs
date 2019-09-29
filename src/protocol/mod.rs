pub mod buffer;
pub mod encoder;
pub mod decoder;
pub mod auth;

use buffer::reader::BufferReader;

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

pub fn write_message<T: encoder::Encoder>(msg: &mut T) {
    let message = match msg.encode() {
        Ok(msg) => msg,
        Err(_) => return
    };

    let length = message.length();
    println!("sending message length={}", length);
}

#[cfg(test)]
pub mod test;
