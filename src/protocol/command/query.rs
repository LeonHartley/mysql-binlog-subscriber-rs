use crate::protocol::buffer::{Buffer, writer::BufferWriter};
use crate::protocol::encoder::{EncodeErr, Encoder};

pub struct Query {
    pub query: String
}

impl Encoder for Query {
    fn encode(&mut self) -> Result<Buffer, EncodeErr> {
        Ok(Buffer::empty()
            .write_i32(3, 1)
            .write_str_no_len(&self.query))
    }
}