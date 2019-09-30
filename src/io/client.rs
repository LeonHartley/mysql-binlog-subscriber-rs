use std::net::{TcpStream};
use std::io::{Read};
use crate::protocol::{encoder::Encoder, decoder::Decoder};
use crate::protocol::response::Response;
use crate::protocol::{auth::{Handshake, HandshakeResponse}, buffer::Buffer, buffer::reader::BufferReader};
use crate::io::{writer::write_message, reader::read_message, reader::read_generic_message, reader::read_buffer};
use crate::protocol::command::query::Query;
use crate::protocol::error::MySqlErr;


pub trait MySqlClient {
    fn sql_msg<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res>;
}

impl MySqlClient for TcpStream {
    fn sql_msg<Req: Encoder, Res: Decoder>(&mut self, msg: &mut Req, sequence: i32) -> Response<Res> {
        write_message(msg, self, sequence);
    
        let mut query_res = [0 as u8; 256];
        match self.read(&mut query_res) {
            Ok(_) => {
                let mut query_buf = Buffer::from_bytes(&query_res);
                println!("{:?}", query_res.to_vec());

                match read_buffer(&mut query_buf) {
                    Ok(mut msg) => match msg.read_u8() {
                        Ok(b) => match b {
                            0xFF /*ERROR*/ => match read_generic_message::<MySqlErr>(&mut msg) {
                                Ok(msg) => Response::Err(msg),
                                Err(e) => Response::InternalErr(format!("Error reading error response, {:?}", e)),
                            },
                            0x00 /*OK*/ => match read_generic_message::<Res>(&mut msg) {
                                Ok(msg) => Response::Ok(msg),
                                Err(e) => Response::InternalErr(format!("Error reading ok response, {:?}", e)),
                            },
                            _ => Response::InternalErr(format!("unknown response type"))
                        }
                        Err(e) => Response::InternalErr(format!("Error reading response type, {:?}", e)),
                    },
                    Err(e) => Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
                }

            },
            Err(e) =>  Response::InternalErr(format!("Error reading response buffer, {:?}", e)),
        }
    }
}