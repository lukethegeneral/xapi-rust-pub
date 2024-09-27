pub mod commands_common;
pub mod commands_main;
pub mod commands_stream;

use serde::Serialize;
use std::error::Error;

use commands_main::Request;
use commands_stream::RequestStream;

pub trait Execute
where
    Self: Serialize,
{
    fn command(&self) -> Result<String, Box<dyn Error>> {
        let json_request = serde_json::to_string(&self)?;
        //println!("\nRequest: {}", json_request);
        Ok(json_request)
    }
}

impl Execute for Request {}
impl Execute for RequestStream {}
