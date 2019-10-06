use std::net::{TcpStream};
use std::io::{Read};
use crate::protocol::{auth::{Handshake, HandshakeResponse, Ok}, encoder::Encoder, decoder::Decoder};
use crate::protocol::response::{Response, QueryResponse};
use crate::protocol::result::{ResultSet, StatementStatus, ColumnDefinition, Eof};
use crate::protocol::command::query::{Query};
use crate::protocol::{buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_message, reader::read_generic_message};
use crate::protocol::error::MySqlErr;
use crate::query::{QueryResultReader, QueryResult};
use crate::options::MySqlOptions;
use crate::protocol::auth::capabilities::{CLIENT_PROTOCOL_41, CLIENT_LONG_FLAG, CLIENT_CONNECT_WITH_DB, CLIENT_SECURE_CONNECTION};

pub trait MySqlClientStream {
    fn authenticate(&mut self, options: &MySqlOptions) -> Response<Ok>;

    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res>;

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>>;
}

impl MySqlClientStream for TcpStream {
    fn authenticate(&mut self, options: &MySqlOptions) -> Response<Ok> {
        let mut buffer = next_buffer(self);

        let msg = match read_message::<Handshake>(&mut buffer) {
            Ok(m) => m,
            Err(err) => return Response::InternalErr(format!("Error decoding handshake message, {:?}", err))
        };

        println!("mysql handshake received\n{:?}", msg);
        println!("attempting authentication, username={}", options.username);

        self.send::<HandshakeResponse, Ok>(&mut HandshakeResponse {
            capability_flags:
            CLIENT_PROTOCOL_41 |
                CLIENT_LONG_FLAG |
                CLIENT_CONNECT_WITH_DB |
                CLIENT_SECURE_CONNECTION,
            max_packet_size: 0,
            character_set: msg.character_set,
            reserved: vec! {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
            auth_data: vec! {},
            username: options.username.clone(),
            database: "information_schema".to_string(),
        }, 1)
    }

    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res> {
        write_message(msg, self, sequence);

        read_response(self)
    }

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>> {
        write_message(&mut Query {
            query: query
        }, self, 0);

        let statement_res = match read_response::<StatementStatus>(self) {
            Response::Ok(res) => res,
            _ => return QueryResult::Err(format!("Error reading statement response"))
        };

        let mut column_defs = vec!{};

        for _ in 0..statement_res.column_count {
            let column_msg = read_response::<ColumnDefinition>(self);
            if let Response::Ok(res) = column_msg {
                column_defs.push(res);
            }
        }

        println!("{:?}", column_defs);

        match read_response::<Eof>(self) {
            Response::Eof(_) => match read_response::<ResultSet>(self) {
                Response::Ok(mut res) => Res::parse(&mut res),
                Response::Err(e) => QueryResult::Err(format!("error executing query, {:?}", e)),
                Response::InternalErr(e) => QueryResult::Err(format!("internal error reading result data, {:?}", e)),
                _ => QueryResult::Err(format!("error reading result set"))
            },
            _ => QueryResult::Err(format!("error reading eof"))
        }
    }
}

pub fn next_buffer(stream: &mut TcpStream) -> Buffer {
    let mut bytes = [0 as u8; 4];      
    match stream.read(&mut bytes) {
        Ok(n) => {
            if n == 0 {
                return Buffer::empty();
            }

            let mut buffer = Buffer::from_bytes(&bytes);
            let length = match buffer.read_i32(3) {
                Ok(n) => n as u64,
                Err(_) => return buffer
            };

            println!("length = {}", length);
            
            let _sequence = buffer.read_u8();

            let mut buf = vec![];
            let mut chunk = stream.take(length);
            if let Ok(_) = chunk.read_to_end(&mut buf) {
                println!("{:?}", &buf);
                Buffer::from_bytes(buf.as_slice())
            }  else {
                println!("couldn't read {} bytes", length);
                Buffer::empty()
            }
        }, 
        Err(e) => {
            println!("error: {:?}", e);
            Buffer::empty()
        }
    }
}

pub fn read_response<Res: Decoder>(stream: &mut TcpStream) -> Response<Res> {
    let mut msg = next_buffer(stream);
     match msg.peek() {
        0xFF /*ERROR*/ => {
            msg.skip(1);
            match read_generic_message::<MySqlErr>(&mut msg) {
                Ok(msg) => Response::Err(msg),
                Err(e) => Response::InternalErr(format!("Error reading error response, {:?}", e)),
            }
        },
        0xFE /*EOF*/ => {
            msg.skip(1);
            match read_generic_message::<Eof>(&mut msg) {
                Ok(msg) => Response::Eof(msg),
                Err(e) => Response::InternalErr(format!("Error reading error response, {:?}", e)),
            }
        }
        b /*OK*/ => {
            if b == 0x00 {
                msg.skip(1);
            }

            match read_generic_message::<Res>(&mut msg) {
                Ok(msg) => Response::Ok(msg),
                Err(e) => Response::InternalErr(format!("Error reading ok response, {:?}", e)),
            }
        }
    }
}