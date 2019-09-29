use super::buffer::*;
use super::auth::*;

use crate::io::reader::read_message;

#[test]
pub fn test_handshake_decode() {
    let msg: Vec<u8> = vec! { 93, 0, 0, 0, 10, 53, 46, 53, 46, 53, 45, 49, 48, 46, 51, 46, 49, 50, 45, 77, 97, 114, 105, 97, 68, 66, 45, 108, 111, 103, 0, 42, 0, 0, 0, 108, 93, 108, 41, 38, 126, 109, 39, 0, 254, 247, 33, 2, 0, 191, 129, 21, 0, 0, 0, 0, 0, 0, 7, 0,
        0, 0, 120, 74, 72, 103, 79, 56, 103, 126, 102, 99, 62, 73, 0, 109, 121, 115, 113, 108, 95, 110, 97, 116, 105, 118, 101, 95, 112, 97, 115, 115, 119, 111, 114, 100, 0 };

    let mut buffer = Buffer::from_bytes(msg.as_slice());
    let msg = match read_message::<Handshake>(&mut buffer) {
        Ok(msg) => msg,
        Err(e) => panic!("error, {:?}", e)
    };

    assert_eq!(msg.protocol_version, 10);
    assert_eq!(msg.server_version, "5.5.5-10.3.12-MariaDB-log");
    assert_eq!(msg.connection_id, 42);
    assert_eq!(msg.auth_plugin_data, "l]l)&~m\'");
    assert_eq!(msg.filter, 0);
    assert_eq!(msg.character_set, 33);
}