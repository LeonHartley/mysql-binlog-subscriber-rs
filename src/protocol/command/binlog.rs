use crate::protocol::buffer::{Buffer, writer::BufferWriter};
use crate::protocol::encoder::{EncodeErr, Encoder};

pub struct DumpBinaryLog {
    pub server_id: i64,
    pub position: i64,
    pub file: String
}

impl Encoder for DumpBinaryLog {
    fn encode(&mut self) -> Result<Buffer, EncodeErr> {
        Ok(Buffer::empty()
            .write_i32(18, 1)
            .write_i64(self.position, 4)
            .write_i32(0, 2)
            .write_i64(self.server_id, 4)
            .write_str_no_len(&self.file))
    }
}