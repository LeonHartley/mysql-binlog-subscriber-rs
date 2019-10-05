use std::net::TcpStream;
use crate::io::stream::MySqlClientStream;
use crate::protocol::{auth::{Handshake, HandshakeResponse, Ok}, encoder::Encoder, decoder::Decoder};
use crate::protocol::response::{Response, QueryResponse};
use crate::protocol::result::{ResultSet, StatementStatus, ColumnDefinition, Eof};
use crate::protocol::command::query::{Query};
use crate::protocol::{buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_message, reader::read_generic_message, reader::read_buffer};
use crate::protocol::error::MySqlErr;
use crate::query::{QueryResultReader, QueryResult};
use crate::options::MySqlOptions;

pub struct MySqlClient {
    pub stream: TcpStream
}

impl Into<MySqlClient> for TcpStream {
    fn into(self) -> MySqlClient {
        MySqlClient {
            stream: self
        }
    }
}

impl MySqlClientStream for MySqlClient {
    fn authenticate(&mut self, options: &MySqlOptions) -> Response<Ok> {
        self.stream.authenticate(options)
    }

    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res> {
        self.stream.send(msg, sequence)
    }

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>> {
        self.stream.query(query)
    }
}