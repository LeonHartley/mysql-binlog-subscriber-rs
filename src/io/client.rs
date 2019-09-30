use std::net::{TcpStream};
use std::io::{Read};
use crate::protocol::{encoder::Encoder, decoder::Decoder};
use crate::protocol::response::{Response, QueryResponse};
use crate::protocol::result::{ResultSet, StatementStatus};
use crate::protocol::command::query::{Query};
use crate::protocol::{buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_generic_message, reader::read_buffer};
use crate::protocol::error::MySqlErr;


pub trait MySqlClient {
    fn send<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res>;

    fn query(&mut self, query: String) -> QueryResponse;
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

    fn query(&mut self, query: String) -> QueryResponse {
        write_message(&mut Query {
            query: query
        }, self, 0);
    
        let mut bytes = [0 as u8; 256];
        let mut buffer = Buffer::from_bytes(&bytes);

        let statement_res = match self.read(&mut bytes) {
            Ok(_) => read_response::<StatementStatus>(&mut buffer),
            Err(e) => Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
        };

        if let Response::Ok(_) = statement_res {
            match read_response::<ResultSet>(&mut buffer) {
                Response::Ok(res) => QueryResponse::Ok(res),
                Response::Err(e) => QueryResponse::Err(e),
                Response::InternalErr(e) => QueryResponse::InternalErr(e),
                _ => QueryResponse::InternalErr(format!("error reading result set"))
            }
        } else {
            QueryResponse::InternalErr(format!("error reading statement response, {:?}", statement_res))
        }
    }
}

fn read_response<Res: Decoder>(buffer: &mut Buffer) -> Response<Res> {
    match read_buffer(buffer) {
        Ok(mut msg) => match msg.read_u8() {
            Ok(b) => match b {
                0xFF /*ERROR*/ => match read_generic_message::<MySqlErr>(&mut msg) {
                    Ok(msg) => Response::Err(msg),
                    Err(e) => Response::InternalErr(format!("Error reading error response, {:?}", e)),
                },
                0xFE /*EOF*/ => Response::Eof,
                _ /*OK*/ => match read_generic_message::<Res>(&mut msg) {
                    Ok(msg) => Response::Ok(msg),
                    Err(e) => Response::InternalErr(format!("Error reading ok response, {:?}", e)),
                }
            }
            Err(e) => Response::InternalErr(format!("Error reading response type, {:?}", e)),
        },
        Err(e) => Response::InternalErr(format!("Error reading response buffer, {:?}", e))
    }   
}