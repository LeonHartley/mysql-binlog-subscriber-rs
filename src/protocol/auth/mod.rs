pub struct Handshake {
    ProtocolVersion: u8,
    ServerVersion: String,
    ConnectionId: i32,
    AuthPluginDataLength: i64
    Filter: u8,
    CapabilityFlag: i16,
    CharacterSet: u8,
    Status: i16,
    CapabilityFlags: i16,
}

pub struct HandshakeResponse {
    CapabilityFlags: i32,
    MaxPacketSize: i32,
    CharacterSet: u8,
    Reserved: [u8, 23],
    Username: String,
    AuthData: Vec<u8>,
    Database: String
}