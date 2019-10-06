use std::io::Read;
use crate::client::MySqlClient;
use crate::protocol::command::binlog::DumpBinaryLog;
use crate::protocol::auth::Ok;
use crate::query::MasterStatus;
use crate::io::stream::{next_buffer, MySqlClientStream};
use crate::query::QueryResult;
use crate::protocol::{buffer::reader::BufferReader, response::Response};
use crate::protocol::event::EventHeader;
use crate::io::stream::read_response;

pub trait MySqlBinlogStream {
    // eventually this will take a handler as an argument 
    fn binlog_listen(&mut self);
}

impl MySqlBinlogStream for MySqlClient {
    fn binlog_listen(&mut self) {
        let mut binlog_connected = false;

        let master_status = match self.query::<MasterStatus>("SHOW MASTER STATUS;".to_string()) {
            QueryResult::Ok(res) => res,
            QueryResult::Err(e) => panic!("Error executing query: {}", e),
        };

        println!("{:?}", master_status);

        // flush the read buffer (hack for now till we handle all msgs)
        let mut bytes = [0 as u8; 1024*3];
        let _ = self.stream.read(&mut bytes);

        match self.send::<DumpBinaryLog, Ok>(&mut DumpBinaryLog {
            server_id: 2, 
            file: master_status.binlog_file,
            position: master_status.binlog_position
        }, 0) {
            Response::Ok(_) => {
                binlog_connected = true;
                println!("binlog connected")
            },
            Response::Err(e) => println!("error sending binlog command: {:?}", e),
            Response::InternalErr(msg) => println!("error: {}", msg),
            Response::Eof(eof) => println!("eof {:?}", eof)
        };

        while binlog_connected {
            let mut buffer = next_buffer(&mut self.stream);
            println!("read bytes: {}", buffer.readable_bytes());

            match read_response::<EventHeader>(&mut buffer) {
                Response::Ok(header) => {
                    println!("got an event, type = {:?}, bytes left: {}", header, buffer.readable_bytes());
                },
                _ => println!("got something else")
            };

            std::thread::sleep(std::time::Duration::from_millis(1))
        }
    }
}