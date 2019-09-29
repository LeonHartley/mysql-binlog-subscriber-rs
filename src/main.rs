pub mod protocol;
pub mod io;

pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read};

    use super::protocol::{auth::{Handshake, HandshakeResponse}, buffer::Buffer};
    use super::io::{writer::write_message, reader::read_message};
    use super::protocol::error::MySqlErr;
    use super::protocol::auth::capabilities::{CLIENT_PROTOCOL_41, CLIENT_LONG_FLAG, CLIENT_CONNECT_WITH_DB};

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
                            capability_flags: CLIENT_PROTOCOL_41 | CLIENT_LONG_FLAG | CLIENT_CONNECT_WITH_DB,
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
                                let mut auth_buf = Buffer::from_bytes(&auth_res);
                                match read_message::<MySqlErr>(&mut auth_buf) {
                                    Ok(msg) => println!("{:?}", msg),
                                    Err(e) => println!("err parsing {:?}", e)
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
