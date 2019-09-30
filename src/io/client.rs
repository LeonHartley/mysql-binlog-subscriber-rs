use std::net::{TcpStream};
use std::io::{Read};
use crate::protocol::{encoder::Encoder, decoder::Decoder};
use crate::protocol::response::{Response, QueryResponse};
use crate::protocol::result::{ResultSet, StatementStatus, ColumnDefinition, Eof};
use crate::protocol::command::query::{Query};
use crate::protocol::{buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_generic_message, reader::read_buffer};
use crate::protocol::error::MySqlErr;
use crate::query::{QueryResultReader, QueryResult};

pub trait MySqlClient {
    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res>;

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>>;
}

impl MySqlClient for TcpStream {
    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res> {
        write_message(msg, self, sequence);
    
        let mut query_res = [0 as u8; 256];
        match self.read(&mut query_res) {
            Ok(_) => read_response(&mut Buffer::from_bytes(&query_res)),
            Err(e) =>  Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
        }
    }

    fn query<Res: QueryResultReader>(&mut self, query: String) -> QueryResult<Box<Res>> {
        write_message(&mut Query {
            query: query
        }, self, 0);
    
        let mut bytes = [0 as u8; 256];      
   
        let query_response = match self.read(&mut bytes) {
            Ok(_) => {
                println!("stream read: {:?}", bytes.to_vec());
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
            _ => QueryResult::Err(format!("error reading result data"))
        }
    }
}

fn read_response<Res: Decoder>(buffer: &mut Buffer) -> Response<Res> {
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