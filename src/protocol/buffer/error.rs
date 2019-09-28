#[derive(Debug)]
pub enum IoErr {
    ReadErr(String),
    WriteErr(String)
}