use super::buffer::{Buffer, reader::BufferReader};
use super::decoder::{DecodeErr, Decoder};

#[derive(Debug)]
pub struct StatementStatus {
    pub column_count: i32   
}

#[derive(Debug)]
pub struct Eof {
    pub warnings: i16,
    pub status_flags: i16,
}

impl Decoder for StatementStatus {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(StatementStatus{
            column_count: match buffer.read_i32(1) {
                Ok(cols) => cols,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding column_count, {:?}", e)))
            }
        }))
    }
}

impl Decoder for Eof {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(Eof{
            warnings: match buffer.read_i16(2) {
                Ok(w) => w,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding warnings, {:?}", e)))
            },
            status_flags: match buffer.read_i16(2) {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding status_flags, {:?}", e)))
            },
        }))
    }
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub catalog: String,
    pub schema: String,
    pub table: String,
    pub org_table: String,
    pub name: String,
    pub org_name: String,
    pub filler_1: u8,
    pub character_set: i16,
    pub column_length: i32,
    pub column_type: u8,
    pub flags: i16,
    pub decimals: u8,
    pub filler_2: i16,
}

impl Decoder for ColumnDefinition {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(ColumnDefinition {
            catalog: match buffer.read_packed_str() {
                Ok(version) => version,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding catalog, {:?}", e)))
            },
            schema: match buffer.read_packed_str() {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding schema, {:?}", e)))
            },
            table: match buffer.read_packed_str() {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding table, {:?}", e)))
            },
            org_table: match buffer.read_packed_str() {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding org_table, {:?}", e)))
            },
            name: match buffer.read_packed_str() {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding name, {:?}", e)))
            },
            org_name: match buffer.read_packed_str() {
                Ok(s) => s,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding org_name, {:?}", e)))
            },
            filler_1: match buffer.read_u8() {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding filler_1, {:?}", e)))
            },
            character_set: match buffer.read_i16(2) {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding character_set, {:?}", e)))
            },
            column_length: match buffer.read_i32(4) {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding column_length, {:?}", e)))
            },
            column_type: match buffer.read_u8() {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding column_type, {:?}", e)))
            },
            flags: match buffer.read_i16(2) {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding flags, {:?}", e)))
            },
            decimals: match buffer.read_u8() {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding decimals, {:?}", e)))
            },
            filler_2: match buffer.read_i16(2) {
                Ok(i) => i,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding column_type, {:?}", e)))
            },
        }))
    }
}

#[derive(Debug)]
pub struct ResultSet {
    data: Vec<String>
}

#[derive(Debug)]
pub struct ResultSetRow {
    values: Vec<String>
}

impl Decoder for ResultSet {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        let mut rows = vec!{};

        while buffer.readable_bytes() > 0 {
            if let Ok(value) = buffer.read_packed_str() {
                rows.push(value);
            }
        };

        Ok(Box::new(ResultSet{
            data: rows
        }))
    }
}