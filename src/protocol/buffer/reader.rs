use super::error::IoErr;
use super::Buffer;
use bytes::{Bytes, Buf};
use byteorder::{BigEndian, LittleEndian, ByteOrder};

pub trait BufferReader {
    fn read_u8(&mut self) -> Result<u8, IoErr>;

    fn read_i32_be(&mut self) -> Result<i32, IoErr>;

    fn read_i16_be(&mut self) -> Result<i16, IoErr>;

    fn read_i32_le(&mut self) -> Result<i32, IoErr>;

    fn read_i16_le(&mut self) -> Result<i16, IoErr>;

    fn read_str_null(&mut self) -> Result<String, IoErr>;
    
    fn read_str_long(&mut self) -> Result<String, IoErr>;
    
    fn read_str(&mut self) -> Result<String, IoErr>;
}

impl BufferReader for Buffer {
    fn read_u8(&mut self) -> Result<u8, IoErr> {
        let data = self.data.as_ref();
        let b = data[0];
        
        self.data.advance(1);
        Ok(b)
    }

    fn read_i32_be(&mut self) -> Result<i32, IoErr> {
        let i = BigEndian::read_i32(&self.data.as_ref());
        self.data.advance(4);

        Ok(i)
    }

    fn read_i16_be(&mut self) -> Result<i16, IoErr> {
        let i = BigEndian::read_i16(&self.data.as_ref());
        self.data.advance(2);

        Ok(i)
    }

    fn read_i32_le(&mut self) -> Result<i32, IoErr> {
        let i = LittleEndian::read_i32(&self.data.as_ref());
        self.data.advance(4);

        Ok(i)
    }

    fn read_i16_le(&mut self) -> Result<i16, IoErr> {
        let i = LittleEndian::read_i16(&self.data.as_ref());
        self.data.advance(2);

        Ok(i)
    }

    fn read_str_null(&mut self) -> Result<String, IoErr> {
        const NULL: u8 = 0;

        let mut bytes_read = 0;
        let mut bytes: Vec<u8> = vec!();
        let data = self.data.as_ref();

        for b in data {
            bytes_read = bytes_read + 1;
            let byte = *b;

            if (byte == NULL) {
                break;
            }
            
            bytes.push(byte);
        }

        self.data.advance(bytes_read);
        Ok(String::from_utf8(bytes).unwrap())
    }
    
    fn read_str_long(&mut self) -> Result<String, IoErr> {
        let mut bytes: Vec<u8> = vec!();
        let length = BigEndian::read_i64(&self.data.as_ref()) as usize;
        self.data.advance(8);
        
        let data = self.data.as_ref();

        for b in data {
            if (bytes.len() == length) {
                break;
            }

            let byte = *b;            
            bytes.push(byte);
        }

        self.data.advance(bytes.len());
        Ok(String::from_utf8(bytes).unwrap())
    }

        
    fn read_str(&mut self) -> Result<String, IoErr> {
        let mut bytes: Vec<u8> = vec!();
        let length = BigEndian::read_i32(&self.data.as_ref()) as usize;
        self.data.advance(4);
        
        let data = self.data.as_ref();

        for b in data {
            if (bytes.len() == length) {
                break;
            }

            let byte = *b;            
            bytes.push(byte);
        }

        self.data.advance(bytes.len());
        Ok(String::from_utf8(bytes).unwrap())
    }
}