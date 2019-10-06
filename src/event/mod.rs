use crate::client::MySqlClient;

pub struct BinlogStream {
    pub client: MySqlClient
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

}