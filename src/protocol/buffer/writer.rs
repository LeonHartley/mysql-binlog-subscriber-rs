use super::error::IoErr;
use super::Buffer;
use bytes::{BufMut, BytesMut};
use byteorder::{BigEndian, LittleEndian, ByteOrder};

pub trait BufferWriter {
    fn write_bytes(mut self, bytes: &Vec<u8>) -> Self;
    
    fn write_i32(mut self, i: i32, length: usize) -> Self;

    fn write_i64(mut self, i: i64, length: usize) -> Self;

    fn write_u8(mut self, b: u8) -> Self;

    fn write_i64_be(mut self, i: i64) -> Self;

    fn write_i32_be(mut self, i: i32) -> Self;

    fn write_i16_be(mut self, i: i16) -> Self;
    
    fn write_i64_le(mut self, i: i64) -> Self;

    fn write_i32_le(mut self, i: i32) -> Self;

    fn write_i16_le(mut self, i: i16) -> Self;

    fn write_str_no_len(mut self, data: &String) -> Self;

    fn write_str(mut self, data: &String) -> Self;

    fn write_str_null(mut self, data: &String) -> Self;

    fn write_str_long(mut self, data: &String) -> Self;
}

impl BufferWriter for Buffer {
    fn write_bytes(mut self, bytes: &Vec<u8>) -> Self {
        self.data.reserve(bytes.len());

        for byte in bytes {
            self.data.put_u8(*byte);
        }

        self
    }

    fn write_i32(mut self, value: i32, length: usize) -> Self {
        for i in 0..length {
            let b = 0x000000FF & (value >> (i << 3));
            self.data.put_u8(b as u8);
        }

        self
    }

    fn write_i64(mut self, value: i64, length: usize) -> Self {
        for i in 0..length {
            let b = 0x00000000000000FF & (value >> (i << 3));
            self.data.put_u8(b as u8);
        }

        self
    }

    fn write_u8(mut self, b: u8) -> Self {
        self.data.reserve(1);
        self.data.put_u8(b);

        self
    }

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

    fn write_i64_le(mut self, i: i64) -> Self {
        self.data.reserve(8);
        self.data.put_i64_le(i);

        self
    }

    fn write_i32_le(mut self, i: i32) -> Self {
        self.data.reserve(4);
        self.data.put_i32_le(i);

        self
    }

    fn write_i16_le(mut self, i: i16) -> Self {
        self.data.reserve(2);
        self.data.put_i16_le(i);

        self
    }

    fn write_str_null(mut self, data: &String) -> Self {
        self.data.reserve(data.len() + 1);
        self.data.put_slice(data.as_bytes());
        self.data.put_u8(0);
        
        self
    }

    fn write_str_long(mut self, data: &String) -> Self {
        self.data.reserve(data.len() + 8);
        self.data.put_i64_be(data.len() as i64);
        self.data.put_slice(data.as_bytes());

        self
    }

    fn write_str(mut self, data: &String) -> Self {
        self.data.reserve(data.len() + 4);
        self.data.put_i32_be(data.len() as i32);
        self.data.put_slice(data.as_bytes());

        self
    }

    fn write_str_no_len(mut self, data: &String) -> Self {
        self.data.reserve(data.len());
        self.data.put_slice(data.as_bytes());

        self
    }

}