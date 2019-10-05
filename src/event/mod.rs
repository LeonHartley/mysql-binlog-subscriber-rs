use std::net::TcpStream;

pub struct BinlogStream {
    connection: Box<TcpStream>,
    binlog_active: bool
}

pub enum BinlogResult {
    Ok,
    Event,
    Err
}

pub trait BinlogReader {
    fn next_event(&mut self) -> BinlogResult;
}

impl BinlogReader for BinlogStream {
    fn next_event(&mut self) -> BinlogResult {
        BinlogResult::Ok
    }
}

impl BinlogStream {
    fn new(stream: Box<TcpStream>) -> BinlogStream {
        BinlogStream {
            connection: stream,
            binlog_active: false
        }
    }

    fn connect(&mut self) -> BinlogResult {
        self.binlog_active = true;

        BinlogResult::Ok
    }
}