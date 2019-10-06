use std::net::TcpStream;
use crate::io::stream::MySqlClientStream;
use crate::protocol::{auth::Ok, encoder::Encoder, decoder::Decoder};
use crate::protocol::response::Response;
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