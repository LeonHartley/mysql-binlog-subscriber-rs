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
        let builder = MySqlClientBuilder::new(MySqlOptions {
            host: "localhost:3306".into(),
            username: "user".into(),
            password: "".into(),
        });
        
        match builder.connect() {
            MySqlConnectResult::Ok(mut client) => match client.query::<MasterStatus>("SHOW MASTER STATUS;".to_string()) {
                QueryResult::Ok(res) => {
                    println!("binlog file: {}, binlog position: {}", res.binlog_file, res.binlog_position);
                },
                QueryResult::Err(e) => println!("Error executing query: {}", e),
            },
            MySqlConnectResult::Err(err) => println!("Error connecting to MySQL: {}", err)
        }
    }
}

fn main() {
    test::connect();
}
