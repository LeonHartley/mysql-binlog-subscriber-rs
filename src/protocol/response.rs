use super::decoder::Decoder;
use super::error::MySqlErr;

pub enum Response<T: Decoder> {
    Ok(Box<T>),
    Eof,
    Err(Box<MySqlErr>),
    InternalErr(String)
}