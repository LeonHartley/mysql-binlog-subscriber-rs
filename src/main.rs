pub mod protocol;
pub mod client {
    use std::net::{TcpStream};
    use std::io::{Read, Write};

    use super::protocol::{read_message, auth::Handshake, buffer::Buffer, decoder::{DecodeErr, Decoder}};

    pub fn connect() {
        let username: String = "root".to_string();

        match TcpStream::connect("localhost:3306") {
            Ok(mut stream) => {
                println!("connected to mysql server on port 3306");

                let mut data = [0 as u8; 128];
                match stream.read(&mut data) {
                    Ok(_) => {
                        println!("{:?}", data.to_vec());
                        let mut buffer = Buffer::from_bytes(&data);
                        let msg = match read_message::<Handshake>(&mut buffer) {
                            Ok(msg) => msg,
                            Err(err) => {
                                println!("Error reading msg, {:?}", err); 
                                return;
                            }
                        };

                        println!("mysql handshake received\n{:?}", msg);
                        println!("attemtping authentication, username={}", username);;
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
