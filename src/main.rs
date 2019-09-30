pub mod protocol;
pub mod io;

pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read};

    use super::protocol::{auth::{Handshake, HandshakeResponse, AuthOk}, result::ResultSet, buffer::Buffer};
    use super::io::{reader::read_message};
    use super::protocol::command::query::Query;
    use super::protocol::error::MySqlErr;
    use super::protocol::auth::capabilities::{CLIENT_PROTOCOL_41,CLIENT_LONG_FLAG,CLIENT_CONNECT_WITH_DB,CLIENT_SECURE_CONNECTION};
    use super::io::client::MySqlClient;
    use super::protocol::response::{Response, QueryResponse};

    pub fn connect() {
        let username = "user".to_string();
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

                        let auth_res = stream.send::<HandshakeResponse, AuthOk>( &mut HandshakeResponse {
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
                        }, 1);

                        match auth_res {
                            Response::Ok(_) => {
                                println!("auth ok");

                                match stream.query("SHOW MASTER STATUS;".to_string()) {
                                    QueryResponse::Ok(res) => println!("result: {:?}", res),
                                    QueryResponse::Err(e) => println!("Error executing query: {}", format_err(&e)),
                                    QueryResponse::InternalErr(e) => println!("error: {:?}", e)
                                };
                            }, 
                            Response::Err(e) => println!("Error authenticating: {}", format_err(&e)),
                            Response::InternalErr(msg) => println!("error: {}", msg),
                            Response::Eof => println!("eof, auth not supported")
                        }
                    },
                    Err(e) => println!("Failed to receive data: {}", e)
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }

        println!("connection terminated");
    }

    fn format_err(msg: &MySqlErr) -> String {
        format!("ERROR {} {}: {}", msg.code, match &msg.state {
            Some(state) => format!("({})", state),
            None => format!("(unknown)")
        }, msg.message)
    }
}

fn main() {
    client::connect();
}
