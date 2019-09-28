pub mod protocol;

use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    println!("mysql data subscriber, written by leonh");

    match TcpStream::connect("localhost:3306") {
        Ok(mut stream) => {
            println!("connected to mysql server on port 3306");

            let mut data = [0 as u8; 512];
            match stream.read(&mut data) {
                Ok(_) => {
                    let mut buffer = protocol::buffer::Buffer::from_bytes(&data);
                    let mut msg = match protocol::read_message::<protocol::auth::Handshake>(&mut buffer) {
                        Ok(msg) => msg,
                        Err(err) => {
                            println!("Error reading msg, {:?}", err); 
                            return;
                        }
                    };

                    println!("read message {:?}", msg);
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
