use super::buffer::{Buffer, writer::BufferWriter, reader::BufferReader};
use super::encoder::{EncodeErr, Encoder};
use super::decoder::{DecodeErr, Decoder};

pub mod capabilities;

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: u8,
    pub server_version: String,
    pub connection_id: i32,
    pub auth_plugin_data: String,
    pub filter: u8,
    pub capability_flag: i16,
    pub character_set: u8,
    pub status: i16,
    pub capability_flags: i16,
}

impl Decoder for Handshake {
    fn decode(buffer: &mut BufferReader) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(Handshake {
            protocol_version: match buffer.read_u8() {
                Ok(version) => version,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding protocol_version, {:?}", e)))
            },
            server_version: match buffer.read_str_null() {
                Ok(version) => version,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding server_version, {:?}", e)))
            },
            connection_id: match buffer.read_i32(4) {
                Ok(id) => id,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding connection_id, {:?}", e)))
            },
            auth_plugin_data: match buffer.read_str_len(8) {
                Ok(plugin_data) => plugin_data,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding auth_plugin_data, {:?}", e)))
            },
            filter: match buffer.read_u8() {
                Ok(filter) => filter,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding filter, {:?}", e)))
            },
            capability_flag: match buffer.read_i16(2) {
                Ok(capability_flag) => capability_flag,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding capability_flag, {:?}", e)))
            },
            character_set: match buffer.read_u8() {
                Ok(character_set) => character_set,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding character_set, {:?}", e)))
            },
            status: match buffer.read_i16(2) {
                Ok(status) => status,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding status, {:?}", e)))
            },
            capability_flags: match buffer.read_i16(2) {
                Ok(capability_flags) => capability_flags,
                Err(e) => return Err(DecodeErr::Err(format!("error capability_flags character_set, {:?}", e)))
            }
        }))
    }
}

#[derive(Debug)]
pub struct HandshakeResponse {
    pub capability_flags: u32,
    pub max_packet_size: i32,
    pub character_set: u8,
    pub reserved: Vec<u8>,
    pub username: String,
    pub auth_data: Vec<u8>,
    pub database: String
}

impl Encoder for HandshakeResponse {
    fn encode(&mut self) -> Result<Buffer, EncodeErr> {
        Ok(Buffer::empty()
            .write_i32(self.capability_flags as i32, 4)
            .write_i32(self.max_packet_size, 4)
            .write_u8(self.character_set)
            .write_bytes(&self.reserved)
            .write_str_null(&self.username)
            .write_bytes(&self.auth_data)
            .write_str_null(&self.database))
    }
}
