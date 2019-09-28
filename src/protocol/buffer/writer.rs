use super::error::IoErr;
use super::Buffer;
use bytes::{BufMut, BytesMut};
use byteorder::{BigEndian, ByteOrder};

pub trait BufferWriter {
    fn write_i32_be(mut self, i: i32) -> Self;

    fn write_i16_be(mut self, i: i16) -> Self;

    fn write_str_null_terminated(mut self, data: String) -> Self;
}

impl BufferWriter for Buffer {
    fn write_i32_be(mut self, i: i32) -> Self {
        self.data.reserve(4);
        self.data.put_i32_be(i);

        self
    }

    fn write_i16_be(mut self, i: i16) -> Self {
        self.data.reserve(2);
        self.data.put_i16_be(i);

        self
    }

    fn write_str_null_terminated(mut self, data: String) -> Self {
        self.data.reserve(data.len() + 1);
        let bytes = data.as_bytes();

        for byte in bytes {
            self.data.put_u8(*byte);
        }

        self.data.put_u8(0);
        
        self
    }
}