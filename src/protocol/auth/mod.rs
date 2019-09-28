use super::buffer::{Buffer, writer::BufferWriter, reader::BufferReader};
use super::encoder::{EncodeErr, Encoder};
use super::decoder::{DecodeErr, Decoder};

pub struct Handshake {
    protocol_version: u8,
    server_version: String,
    connection_id: i32,
    auth_plugin_data: String,
    filter: u8,
    capability_flag: i16,
    character_set: u8,
    status: i16,
    capability_flags: i16,
}

impl Decoder for Handshake {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(Handshake {
            protocol_version: match buffer.read_u8() {
                Ok(version) => version,
                Err(_) => return Err(DecodeErr::Err(String::from("protocol_version")))
            },
            server_version: match buffer.read_str_null() {
                Ok(version) => version,
                Err(_) => return Err(DecodeErr::Err(String::from("server_version")))
            },
            connection_id: match buffer.read_i32_le() {
                Ok(id) => id,
                Err(_) => return Err(DecodeErr::Err(String::from("connection_id")))
            },
            auth_plugin_data: match buffer.read_str_long() {
                Ok(plugin_data) => plugin_data,
                Err(_) => return Err(DecodeErr::Err(String::from("auth_plugin_data")))
            },
            filter: match buffer.read_u8() {
                Ok(filter) => filter,
                Err(_) => return Err(DecodeErr::Err(String::from("filter")))
            },
            capability_flag: match buffer.read_i16_le() {
                Ok(capability_flag) => capability_flag,
                Err(_) => return Err(DecodeErr::Err(String::from("capability_flag")))
            },
            character_set: match buffer.read_u8() {
                Ok(character_set) => character_set,
                Err(_) => return Err(DecodeErr::Err(String::from("character_set")))
            },
            status: match buffer.read_i16_le() {
                Ok(status) => status,
                Err(_) => return Err(DecodeErr::Err(String::from("status")))
            },
            capability_flags: match buffer.read_i16_le() {
                Ok(capability_flags) => capability_flags,
                Err(_) => return Err(DecodeErr::Err(String::from("capability_flags")))
            }
        }))
    }
}

pub struct HandshakeResponse {
    capability_flags: i32,
    max_packet_size: i32,
    character_set: u8,
    reserved: String,
    username: String,
    auth_data: Vec<u8>,
    database: String
}

impl Encoder for HandshakeResponse {
    fn encode(&mut self) -> Result<Buffer, EncodeErr> {
        Ok(Buffer::empty()
            .write_i32_le(self.capability_flags)
            .write_i32_le(self.max_packet_size)
            .write_u8(self.character_set)
            .write_str(&self.reserved)
            .write_str(&self.username)
            .write_bytes(&self.auth_data)
            .write_str(&self.database))
    }
}
