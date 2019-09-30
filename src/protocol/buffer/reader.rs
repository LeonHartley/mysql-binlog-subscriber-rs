use super::error::IoErr;
use super::Buffer;
use byteorder::{BigEndian, LittleEndian, ByteOrder};

pub trait BufferReader {
    fn skip(&mut self, num: usize);

    fn peek(&mut self) -> u8;

    fn readable_bytes(&mut self) -> usize; 

    fn read_u8(&mut self) -> Result<u8, IoErr>;

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, IoErr>;

    fn read_i32_be(&mut self) -> Result<i32, IoErr>;

    fn read_i16_be(&mut self) -> Result<i16, IoErr>;

    fn read_i64_le(&mut self) -> Result<i64, IoErr>;

    fn read_i32_le(&mut self) -> Result<i32, IoErr>;

    fn read_i16_le(&mut self) -> Result<i16, IoErr>;

    fn read_i64(&mut self, len: usize) -> Result<i64, IoErr>;

    fn read_i32(&mut self, len: usize) -> Result<i32, IoErr>;

    fn read_i16(&mut self, len: usize) -> Result<i16, IoErr>;

    fn read_str_null(&mut self) -> Result<String, IoErr>;
    
    fn read_packed_i64(&mut self) -> Result<i64, IoErr>;

    fn read_str(&mut self) -> Result<String, IoErr>;

    fn read_str_len(&mut self, len: usize) -> Result<String, IoErr>;
}

impl BufferReader for Buffer {  
    fn read_packed_i64(&mut self) -> Result<i64, IoErr> {
        let i = match self.read_u8() {
            Ok(b) => b,
            Err(e) => return Err(e)
        };

        if i < 251 {
            Ok(i as i64)
        } else if i == 251 {
            Ok(-1)
        } else if i == 252 {
            match self.read_i64(2) {            
                Ok(b) => Ok(b),
                 Err(e) => return Err(e)
            }
        } else if i == 253 {
            match self.read_i64(3) {            
                Ok(b) => Ok(b),
                Err(e) => return Err(e)
            }
        } else if i == 254 {
            match self.read_i64(8) {            
                Ok(b) => Ok(b),
                Err(e) => return Err(e)
            }
        } else {
            Err(IoErr::ReadErr(format!("failed to read packed number, i = {}", i)))
        }
    }

    fn read_i64(&mut self, len: usize) -> Result<i64, IoErr> {
        let mut result = 0 as u64;

        for i in 0..len {
            match self.read_u8() {
                Ok(n) => result |= (n as u64) << (i << 3),
                Err(e) => return Err(IoErr::ReadErr(format!("failed to read number, length={}, error={:?}", len, e)))
            };
        }

        Ok(result as i64)
    }

    fn read_i32(&mut self, len: usize) -> Result<i32, IoErr> {
        match self.read_i64(len) {
            Ok(n) => Ok(n as i32),
            Err(e) => Err(e)
        }
    }

    fn read_i16(&mut self, len: usize) -> Result<i16, IoErr> {
        match self.read_i64(len) {
            Ok(n) => Ok(n as i16),
            Err(e) => Err(e)
        }
    }

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, IoErr> {
        let mut bytes = vec!{};
        let data = self.data.as_ref();

        for i in 0..len {
            bytes.push(data[i]);
        }

        self.data.advance(len);

        Ok(bytes)
    }

    fn skip(&mut self, num: usize) {
        self.data.advance(num);
    }

    fn peek(&mut self) -> u8 {
        let data = self.data.as_ref();
        data[0]
    }

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

    fn read_i64_le(&mut self) -> Result<i64, IoErr> {
        let i = LittleEndian::read_i64(&self.data.as_ref());
        self.data.advance(8);

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

            if byte == NULL {
                break;
            }
            
            bytes.push(byte);
        }

        self.data.advance(bytes_read);
        
        match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err(IoErr::ReadErr(format!("error reading str, length={}", bytes_read)))
        }
    }
    
    // fn read_str_long(&mut self) -> Result<String, IoErr> {
    //     let mut bytes: Vec<u8> = vec!();
    //     let length = LittleEndian::read_i64(&self.data.as_ref()) as usize;
    //     self.data.advance(8);
        
    //     let data = self.data.as_ref();

    //     for b in data {
    //         if (bytes.len() == length) {
    //             break;
    //         }

    //         let byte = *b;            
    //         bytes.push(byte);
    //     }

    //     self.data.advance(bytes.len());
        
    //     match String::from_utf8(bytes) {
    //         Ok(string) => Ok(string),
    //         Err(_) => Err(IoErr::ReadErr(format!("error reading str, length={}", length)))
    //     }
    // }

        
    fn read_str(&mut self) -> Result<String, IoErr> {
        let mut bytes: Vec<u8> = vec!();
        let length = BigEndian::read_i32(&self.data.as_ref()) as usize;
        self.data.advance(4);
        
        let data = self.data.as_ref();

        for b in data {
            if bytes.len() == length {
                break;
            }

            let byte = *b;            
            bytes.push(byte);
        }

        self.data.advance(bytes.len());

        match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err(IoErr::ReadErr(format!("error reading str, length={}", length)))
        }
    }

    fn read_str_len(&mut self, len: usize) -> Result<String, IoErr> {
        let mut bytes: Vec<u8> = vec!();
        let data = self.data.as_ref();

        for i in 0..len {
            let byte = data[i];    
            bytes.push(byte)
        }

        self.data.advance(bytes.len());

        match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err(IoErr::ReadErr(format!("error reading str, length={}", len)))
        }
    }

    fn readable_bytes(&mut self) -> usize {
        self.data.len()
    }
}