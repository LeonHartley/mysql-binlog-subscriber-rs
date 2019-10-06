# MySQL Binary Log Data Subscriber
This is a library and server created to enable the easy transform of realtime MySQL data. Enabling applications such as serving realtime data via websockets to subscribe to a realtime transformed JSON stream directly from the MySQL binary log. 

## MySQL Client 
This project doesn't use any external MySQL client libraries, it implements the MySQL protocol and uses a raw TCP stream. Example of the MySQL client API exposed by this library:

```rust
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
 ```
