use super::decoder::Decoder;
use super::error::MySqlErr;
use super::result::{Eof,ResultSet};

#[derive(Debug)]
pub enum Response<T: Decoder> {
    Ok(Box<T>),
    Eof(Box<Eof>),
    Err(Box<MySqlErr>),
    InternalErr(String)
}

#[derive(Debug)]
pub enum QueryResponse {
    Ok(Box<ResultSet>),
    Err(Box<MySqlErr>),
    InternalErr(String)
}