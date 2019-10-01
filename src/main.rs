pub mod protocol;
pub mod io;
pub mod query;

pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read};

    use super::protocol::{auth::{Handshake, HandshakeResponse, Ok}, buffer::Buffer, command::binlog::DumpBinaryLog};
    use super::io::{reader::read_message};
    use super::protocol::error::MySqlErr;
    use super::protocol::auth::capabilities::{CLIENT_PROTOCOL_41,CLIENT_LONG_FLAG,CLIENT_CONNECT_WITH_DB,CLIENT_SECURE_CONNECTION};
    use super::io::client::MySqlClient;
    use super::protocol::response::{Response};
    use super::query::{QueryResult, MasterStatus};

    pub fn connect() {
        let username = "user".to_string();
        let database = "cometsrv".to_string();

        let mut binlog_connected: bool = false;

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

                        let auth_res = stream.send::<HandshakeResponse, Ok>( &mut HandshakeResponse {
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

                                match stream.query::<MasterStatus>("SHOW MASTER STATUS;".to_string()) {
                                    QueryResult::Ok(res) => {
                                        println!("binlog file: {}, binlog position: {}", res.binlog_file, res.binlog_position);

                                        // request binlog stream
                                        match stream.send::<DumpBinaryLog, Ok>(&mut DumpBinaryLog {
                                            server_id: 2, 
                                            file: res.binlog_file,
                                            position: res.binlog_position
                                        }, 0) {
                                            Response::Ok(_) => {
                                                binlog_connected = true;
                                                println!("binlog connected")
                                            },
                                            Response::Err(e) => println!("error sending binlog command: {}", format_err(&e)),
                                            Response::InternalErr(msg) => println!("error: {}", msg),
                                            Response::Eof(_) => println!("eof")
                                        };
                                    },
                                    QueryResult::Err(e) => println!("Error executing query: {}", e),
                                };
                            }, 
                            Response::Err(e) => println!("Error authenticating: {}", format_err(&e)),
                            Response::InternalErr(msg) => println!("error: {}", msg),
                            Response::Eof(_) => println!("eof, auth not supported")
                        }
                    },
                    Err(e) => println!("Failed to receive data: {}", e)
                };

                while binlog_connected {
                    let mut data = [0 as u8; 128];
                    match stream.read(&mut data) {
                        Ok(n) => {
                            println!("{:?}", String::from_utf8_lossy(&mut data));
                            if data[0] != 0 {
                                println!("received data {:?}", data.to_vec());
                            }
                        },
                        _=> { println!("lol?")}
                    };

                    std::thread::sleep(std::time::Duration::from_millis(1));
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
