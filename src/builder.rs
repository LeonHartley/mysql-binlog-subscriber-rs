use std::net::TcpStream;
use std::io::Read;
use super::{options::MySqlOptions, client::MySqlClient};
use super::protocol::{auth::Ok, buffer::Buffer, buffer::reader::BufferReader, command::binlog::DumpBinaryLog, event::EventHeader};
use super::io::stream::read_response;
use super::protocol::error::MySqlErr;
use super::io::stream::MySqlClientStream;
use super::query::{QueryResult, MasterStatus};
use super::protocol::response::{Response};

pub struct MySqlClientBuilder {
    options: MySqlOptions
}

pub enum MySqlConnectResult {
    Ok(MySqlClient),
    Err(String)
}

impl MySqlClientBuilder {
    pub fn new(options: MySqlOptions) -> Self {
        Self {
            options: options
        }
    }

    pub fn connect(self) -> MySqlConnectResult {
        match TcpStream::connect(self.options.host.clone()) {
            Ok(mut stream) => match stream.authenticate(&self.options) {
                    Response::Ok(_) => MySqlConnectResult::Ok(stream.into()),
                    Response::Err(e) => MySqlConnectResult::Err(format!("error reading result data, {:?}", e)),
                    Response::InternalErr(e) => MySqlConnectResult::Err(format!("internal error reading data, {}", e)),
                    Response::Eof(_) => MySqlConnectResult::Err(format!("EOF response returned"))
            },
            _ => MySqlConnectResult::Err(format!("Error reading from stream"))
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::{MySqlClientBuilder, MySqlOptions};
    
    #[test]
    pub fn test_builder_options() {
        let host = "localhost".to_string();
        let username = "root".to_string();
        let password = "test".to_string();

        let builder = MySqlClientBuilder::new(MySqlOptions {
                host: host.clone(),
                username: username.clone(),
                password: password.clone()
            });

        let options = builder.options;

        assert_eq!(options.host, host);
        assert_eq!(options.username, username);
        assert_eq!(options.password, password);
    }
}