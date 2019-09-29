pub mod protocol;
pub mod io;

pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read, Write};

    use super::protocol::{auth::{Handshake, HandshakeResponse}, buffer::Buffer, decoder::{DecodeErr, Decoder}};
    use super::io::{writer::write_message, reader::read_message};

    pub fn connect() {
        let username = "root".to_string();
        let database = "database".to_string();

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
                            capability_flags: 0,
                            max_packet_size: 0,
                            character_set: msg.character_set,
                            reserved: vec!{0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0},
                            username: username,
                            auth_data: vec!{},
                            database: database,
                        }, &mut stream);
                        
                        let mut auth_res = [0 as u8; 128];
                        match stream.read(&mut auth_res) {
                            Ok(_) => {
                                println!("auth response: {:?}", String::from_utf8_lossy(&auth_res));
                            },                    
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
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
