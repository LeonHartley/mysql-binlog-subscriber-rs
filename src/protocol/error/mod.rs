use super::buffer::{Buffer, reader::BufferReader};
use super::decoder::{DecodeErr, Decoder};

#[derive(Debug)]
pub struct MySqlErr {
    code: i16,
    state: String,
    message: String,
}

impl Decoder for MySqlErr {
    fn decode(buffer: &mut BufferReader) -> Result<Box<Self>, DecodeErr> {
        buffer.skip(1);

        Ok(Box::new(MySqlErr {
            code: match buffer.read_i16(2) {
                Ok(code) => code,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding error code, {:?}", e)))
            },
            state: {
                buffer.skip(1);
                match buffer.read_str_len(5) {
                    Ok(code) => code,
                    Err(e) => return Err(DecodeErr::Err(format!("error decoding error state, {:?}", e)))
                }
            },
            message: match buffer.read_str_null() {
                Ok(message) => message,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding error message, {:?}", e)))
            }
        }))
    }
}