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
            match client.query::<MasterStatus>("SHOW MASTER STATUS;".to_string()) {
                QueryResult::Ok(res) => {
                    println!("binlog file: {}, binlog position: {}", res.binlog_file, res.binlog_position);
                },
                QueryResult::Err(e) => println!("Error executing query: {}", e),
            };
        }
    }
}

fn main() {
    test::connect();
}
