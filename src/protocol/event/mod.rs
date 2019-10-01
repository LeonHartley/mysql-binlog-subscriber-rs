use super::buffer::{Buffer, reader::BufferReader};
use super::decoder::{DecodeErr, Decoder};

#[derive(Debug)]
pub enum EventType {
    Unknown = 0,
    Start = 1,
    Query = 2,
    Stop = 3,
    Xid = 16,
    TableMap = 19,
    WriteRows = 23,
    UpdateRows = 24,
    DeleteRows = 25,
    Heartbeat = 26,
}

#[derive(Debug)]
pub struct EventHeader {
    pub timestamp: i64,
    pub event_type: EventType,
    pub server_id: i64,
    pub length: usize,
    pub next_position: i64,
    pub flags: i16,
}

impl Decoder for EventHeader {
    fn decode(buffer: &mut Buffer) -> Result<Box<Self>, DecodeErr> {
        Ok(Box::new(EventHeader {
            timestamp: match buffer.read_i64(4) {
                Ok(timestamp) => timestamp,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding timestamp, {:?}", e)))
            },
            event_type: match buffer.read_u8() {
                Ok(e) => e.into(),
                Err(e) => return Err(DecodeErr::Err(format!("error decoding event_type, {:?}", e)))
            },
            server_id: match buffer.read_i64(4) {
                Ok(id) => id,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding server_id, {:?}", e)))
            },
            length: match buffer.read_i64(4) {
                Ok(length) => length as usize,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding length, {:?}", e)))
            },
            next_position: match buffer.read_i64(4) {
                Ok(next_position) => next_position,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding next_position, {:?}", e)))
            },
            flags: match buffer.read_i16(2) {
                Ok(flags) => flags,
                Err(e) => return Err(DecodeErr::Err(format!("error decoding flags, {:?}", e)))
            },
        }))
    }
}

impl Into<EventType> for u8 {
    fn into(self) -> EventType {
        match self {
            1 => EventType::Start,
            2 => EventType::Query,
            3 => EventType::Stop,
            16 => EventType::Xid,
            19 => EventType::TableMap,
            23 => EventType::WriteRows,
            24 => EventType::UpdateRows,
            25 => EventType::DeleteRows,
            25 => EventType::Heartbeat,
            _ => EventType::Unknown
        }
    }
}