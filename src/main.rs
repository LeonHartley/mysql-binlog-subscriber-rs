pub mod protocol;
pub mod io;
pub mod query;
pub mod binlog;
pub mod builder;
pub mod options;
pub mod client;

mod test {
    use super::options::MySqlOptions;
    use super::builder::{MySqlClientBuilder, MySqlConnectResult};
    use super::binlog::MySqlBinlogStream;

    pub fn connect() {
        let mut client = match MySqlClientBuilder::new(MySqlOptions {
            host: "localhost:3306".into(),
            username: "user".into(),
            password: "".into(),
        }).connect() {
            MySqlConnectResult::Ok(client) => client,
            MySqlConnectResult::Err(msg) => panic!("couldn't connect to mysql, error: {}", msg)
        };
        
        println!("starting binlog stream..");

        client.binlog_listen()
    }
}

fn main() {
    test::connect();
}
