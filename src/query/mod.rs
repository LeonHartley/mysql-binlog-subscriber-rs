use crate::protocol::result::ResultSet;

#[derive(Debug)]
pub enum QueryResult<T> {
    Ok(T),
    Err(String)
}

pub trait QueryResultReader {
    fn parse(result_set: &mut ResultSet) -> QueryResult<Box<Self>>;
}

#[derive(Debug)]
pub struct MasterStatus {
    pub binlog_file: String,
    pub binlog_position: i64
}

impl QueryResultReader for MasterStatus {
    fn parse(result_set: &mut ResultSet) -> QueryResult<Box<Self>> {
        QueryResult::Ok(Box::new(MasterStatus {
            binlog_file: result_set.data[0].clone(),
            binlog_position: match result_set.data[1].parse::<i64>() {
                Ok(n) => n,
                Err(e) => return QueryResult::Err(format!("error reading binlog_position, {:?}", e))
            }
        }))
    }
}