pub mod protocol;
pub mod io;
pub mod query;
pub mod event;
pub mod builder;
pub mod options;
pub mod client;

mod test {
    use std::net::TcpStream;
    use std::io::Read;

    use super::protocol::{auth::Ok, buffer::Buffer, buffer::reader::BufferReader, command::binlog::DumpBinaryLog, event::EventHeader};
    use super::io::stream::read_response;
    use super::protocol::error::MySqlErr;
    use super::io::stream::MySqlClientStream;
    use super::query::{QueryResult, MasterStatus};
    use super::protocol::response::{Response};
    use super::options::MySqlOptions;
    use super::builder::{MySqlClientBuilder, MySqlConnectResult};

    pub fn connect() {
        let mut binlog_connected: bool = false;

        let builder = MySqlClientBuilder::new(MySqlOptions {
            host: "localhost:3306".into(),
            username: "user".into(),
            password: "".into(),
        });
        
        if let MySqlConnectResult::Ok(mut client) = builder.connect() {
            while binlog_connected {
                let mut data = [0 as u8; 1024*100];
                match client.stream.read(&mut data) {
                    Ok(n) => {
                        let mut buffer = Buffer::from_bytes(&data);
                        while buffer.readable_bytes() > 0 {
                            match read_response::<EventHeader>(&mut buffer) {
                                Response::Ok(header) => {
                                    
                                    println!("got an event, type = {:?}", header);
                                },
                                _ => println!("got something else")
                            };
                        }
                    }
                    _ => { println!("err reading binlog") }
                };

                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
}

fn main() {
    test::connect();
}
