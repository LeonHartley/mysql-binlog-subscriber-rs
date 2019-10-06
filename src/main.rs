pub mod protocol;
pub mod io;
pub mod query;
pub mod event;
pub mod builder;
pub mod options;
pub mod client;

mod test {
    use super::io::stream::MySqlClientStream;
    use super::query::{QueryResult, MasterStatus};
    use super::options::MySqlOptions;
    use super::builder::{MySqlClientBuilder, MySqlConnectResult};

    pub fn connect() {
        let mut client = match MySqlClientBuilder::new(MySqlOptions {
            host: "localhost:3306".into(),
            username: "user".into(),
            password: "".into(),
        }).connect() {
            MySqlConnectResult::Ok(client) => client,
            MySqlConnectResult::Err(msg) => panic!("couldn't connect to mysql, error: {}", msg)
        };
        
        let master_status = match client.query::<MasterStatus>("SHOW MASTER STATUS;".to_string()) {
            QueryResult::Ok(res) => res,
            QueryResult::Err(e) => panic!("Error executing query: {}", e),
        };

        println!("binlog file: {}, binlog position: {}", 
            master_status.binlog_file,
            master_status.binlog_position);
    }
}

fn main() {
    test::connect();
}
