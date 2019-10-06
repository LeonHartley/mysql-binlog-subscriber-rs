use crate::protocol::buffer::{Buffer, reader::BufferReader};
use crate::protocol::decoder::{Decoder, DecodeErr};

// pub fn read_buffer(buffer: &mut Buffer) -> Result<Buffer, DecodeErr> {
//     let length = match buffer.read_i32(3) {
//         Ok(n) => n,
//         Err(e) => return Err(DecodeErr::Err(format!("failed to decode length, {:?}", e)))
//     };

//     let _sequence = match buffer.read_u8() {
//         Ok(seq) => seq,
//         Err(e) => return Err(DecodeErr::Err(format!("failed to decode sequence, {:?}", e)))
//     };

//     println!("length={}, readableBytes={}", length, buffer.readable_bytes());

//     if length < 0 {
//         Err(DecodeErr::Err(format!("failed to decode message, length={}", length)))
//     } else {
//         match buffer.read_bytes(length as usize) {
//             Ok(bytes) => {
//                 println!("read bytes: {}", length);
//                 Ok(Buffer::from_bytes(bytes.as_ref()))
//             },
//             Err(e) => Err(DecodeErr::Err(format!("failed to decode length, {:?}", e)))
//         }
//     }
// }

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