pub mod writer;
pub mod reader;

use super::protocol::{encoder::Encoder, decoder::Decoder};

// pub trait MySqlClient {
//     fn send<Req: Encoder, Res: Decoder, F>(msg: &mut Req, handler: F) 
//         where F: Fn(&T, Self) -> Result<
// }