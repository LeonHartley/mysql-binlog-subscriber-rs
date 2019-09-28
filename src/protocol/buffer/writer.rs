use super::error::IoErr;
use super::Buffer;
use bytes::{BufMut, BytesMut};
use byteorder::{BigEndian, ByteOrder};

pub trait BufferWriter {
    fn write_i64_be(mut self, i: i64) -> Self;

    fn write_i32_be(mut self, i: i32) -> Self;

    fn write_i16_be(mut self, i: i16) -> Self;

    fn write_str_null(mut self, data: String) -> Self;

    fn write_str_long(mut self, data: String) -> Self;
}

impl BufferWriter for Buffer {
    fn write_i64_be(mut self, i: i64) -> Self {
        self.data.reserve(8);
        self.data.put_i64_be(i);

        self
    }

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

    fn write_str_null(mut self, data: String) -> Self {
        self.data.reserve(data.len() + 1);
        self.data.put_slice(data.as_bytes());
        self.data.put_u8(0);
        
        self
    }

    fn write_str_long(mut self, data: String) -> Self {
        self.data.reserve(data.len() + 8);
        self.data.put_i64_be(data.len() as i64);
        self.data.put_slice(data.as_bytes());

        self
    }
}