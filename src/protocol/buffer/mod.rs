use byteorder::BigEndian;
use bytes::{BufMut, BytesMut};
use bytes::ByteOrder;

pub struct Buffer {
    data: BytesMut
}

impl Buffer {
    pub fn empty() -> Buffer {
        Buffer {
            data: BytesMut::new()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Buffer {
        Buffer {
            data: BytesMut::from(bytes)
        }
    }

    pub fn into_bytes(&mut self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

pub mod error;
pub mod reader;
pub mod writer;

#[cfg(test)]
pub mod test;