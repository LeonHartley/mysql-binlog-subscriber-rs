pub mod protocol;
pub mod io;

pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read};

    use super::protocol::{auth::{Handshake, HandshakeResponse}, buffer::Buffer, buffer::reader::BufferReader, decoder::{DecodeErr, Decoder}};
    use super::io::{writer::write_message, reader::read_message, reader::read_generic_message, reader::read_buffer};
    use super::protocol::error::MySqlErr;
    use super::protocol::auth::capabilities::{CLIENT_PROTOCOL_41,CLIENT_LONG_FLAG,CLIENT_CONNECT_WITH_DB,CLIENT_SECURE_CONNECTION};
    
    pub fn connect() {
        let username = "root".to_string();
        let database = "cometsrv".to_string();

        match TcpStream::connect("localhost:3306") {
            Ok(mut stream) => {
                println!("connected to mysql server on port 3306");

                let mut data = [0 as u8; 128];
                match stream.read(&mut data) {
                    Ok(_) => {
                        let mut buffer = Buffer::from_bytes(&data);
                        let msg = match read_message::<Handshake>(&mut buffer) {
                            Ok(msg) => msg,
                            Err(err) => {
                                println!("Error reading msg, {:?}", err); 
                                return;
                            }
                        };

                        println!("mysql handshake received\n{:?}", msg);
                        println!("attempting authentication, username={}", username);

                        write_message(&mut HandshakeResponse {
                            capability_flags: 
                                CLIENT_PROTOCOL_41 | 
                                CLIENT_LONG_FLAG | 
                                CLIENT_CONNECT_WITH_DB | 
                                CLIENT_SECURE_CONNECTION,
                            max_packet_size: 0,
                            character_set: msg.character_set,
                            reserved: vec!{0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0},
                            username: username,
                            auth_data: vec!{},
                            database: database,
                        }, &mut stream, 1);
                        
                        let mut auth_res = [0 as u8; 128];
                        match stream.read(&mut auth_res) {
                            Ok(_) => {
                                // todo: split this up
                                let mut auth_buf = Buffer::from_bytes(&auth_res);
                                
                                match read_buffer(&mut auth_buf) {
                                    Ok(mut msg) => match msg.read_u8() {
                                        Ok(res_type) => match res_type {
                                            0xFF => match read_generic_message::<MySqlErr>(&mut msg) {
                                                Ok(msg) => println!("{:?}", msg),
                                                Err(e) => println!("error parsing error {:?}", e)
                                            },
                                            0xFE => println!("0xFE"),
                                            _ => println!("msg {}", res_type),
                                        } ,
                                        Err(e) => println!("error parsing auth response {:?}", e)
                                    }
                                    Err(e) => println!("error parsing auth response {:?}", e)
                                }
                            },                    
                            Err(e) => {
                                println!("Failed to receive data: {}", e)
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }

        println!("connection terminated");
    }
}

fn main() {
    client::connect();
}
