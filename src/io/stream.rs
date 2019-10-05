use std::net::{TcpStream};
use std::io::{Read};
use crate::protocol::{auth::{Handshake, HandshakeResponse, Ok}, encoder::Encoder, decoder::Decoder};
use crate::protocol::response::{Response, QueryResponse};
use crate::protocol::result::{ResultSet, StatementStatus, ColumnDefinition, Eof};
use crate::protocol::command::query::{Query};
use crate::protocol::{buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_message, reader::read_generic_message, reader::read_buffer};
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
        let mut data = [0 as u8; 128];
        match self.read(&mut data) {
            Ok(_) => {
                let mut buffer = Buffer::from_bytes(&data);
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
            },
            _ => Response::InternalErr("error reading from stream".into())
        }
    }

    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res> {
        write_message(msg, self, sequence);
    
        let mut query_res = [0 as u8; 1024];
        match self.read(&mut query_res) {
            Ok(_) => read_response(&mut Buffer::from_bytes(&query_res)),
            Err(e) =>  Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
        }
    }

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>> {
        write_message(&mut Query {
            query: query
        }, self, 0);
    
        let mut bytes = [0 as u8; 2048];      
        let query_response = match self.read(&mut bytes) {
            Ok(_) => {
                // println!("stream read: {:?}", bytes.to_vec());
                let mut buffer = Buffer::from_bytes(&bytes);
                let statement_res = read_response::<StatementStatus>(&mut buffer);

                if let Response::Ok(res) = statement_res {
                    let mut column_defs = vec!{};

                    for _ in 0..res.column_count {
                        let column_msg = read_response::<ColumnDefinition>(&mut buffer);
                        if let Response::Ok(res) = column_msg {
                            column_defs.push(res);
                        }
                    }

                    println!("{:?}", column_defs);

                    match read_response::<Eof>(&mut buffer) {
                        Response::Eof(_) => match read_response::<ResultSet>(&mut buffer) {
                            Response::Ok(res) => QueryResponse::Ok(res),
                            Response::Err(e) => QueryResponse::Err(e),
                            Response::InternalErr(e) => QueryResponse::InternalErr(e),
                            _ => QueryResponse::InternalErr(format!("error reading result set"))
                        },
                        _ => QueryResponse::InternalErr(format!("error reading eof"))
                    }
                } else {
                    QueryResponse::InternalErr(format!("error reading statement response, {:?}", statement_res))
                }
            }
            Err(e) => QueryResponse::InternalErr(format!("Error reading from stream, {:?}", e)),
        };
        
        match query_response {
            QueryResponse::Ok(mut res) => Res::parse(&mut res),
            QueryResponse::Err(e) => QueryResult::Err(format!("error reading result data, {:?}", e)),
            QueryResponse::InternalErr(e) => QueryResult::Err(format!("internal error reading result data, {}", e))
        }
    }
}

pub fn read_response<Res: Decoder>(buffer: &mut Buffer) -> Response<Res> {
    match read_buffer(buffer) {
        Ok(mut msg) => match msg.peek() {
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
        }, 
         Err(e) => Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
    }   
}