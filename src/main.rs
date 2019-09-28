use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    println!("mysql data subscriber, written by leonh");

    match TcpStream::connect("localhost:3306") {
        Ok(mut stream) => {
            println!("connected to mysql server on port 3306");

            // let mut data = [0 as u8; 512];
            // match stream.read(&mut data) {
            //     Ok(_) => {
            //         let mut buffer = protocol::buffer::Buffer::from_bytes(&data);
            //         let mut msg = protocol::auth::Handshake::decode(&mut buffer);
            //     },
            //     Err(e) => {
            //         println!("Failed to receive data: {}", e);
            //     }
            // }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("connection terminated");
}
