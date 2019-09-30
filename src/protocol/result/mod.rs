use super::buffer::{Buffer, reader::BufferReader};
use super::decoder::{DecodeErr, Decoder};

#[derive(Debug)]
pub struct ResultSet {
    rows: Vec<ResultSetRow>
}

#[derive(Debug)]
pub struct ResultSetRow {
    values: Vec<String>
}

impl Decoder for ResultSet {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        let mut rows = vec!{};
        println!("lol??? {}, {:?}", buffer.readable_bytes(), buffer.read_u8());
        while buffer.readable_bytes() > 1 {
            let length = if let Ok(len) = buffer.read_packed_i64() {
                len
            } else {
                return Err(DecodeErr::Err(format!("failed to read length of resultset value")))
            };

println!("lol?");
            let value = buffer.read_str_len(length as usize);
            println!("{:?}", value);
        };

        Ok(Box::new(ResultSet{
            rows: rows
        }))
    }
}