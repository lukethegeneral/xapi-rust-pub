pub mod commands_main;
pub mod commands_stream;
use serde::{Deserialize, Serialize};
use std::error::Error;

use commands_main::*;
//use commands_main::Request;
use self::commands_stream::RequestStream;

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

/*
pub trait GetResult
where
    Self: Serialize,
{
    fn get_response<T>(&self) -> Result<&T, Box<dyn Error>>; 
}

impl GetResult for GetCommissionDefResponse {
    fn get_response<GetCommissionDefResponse>(&self) -> Result<&Self, Box<dyn Error>> {
        Ok(self)
    }
}
*/

impl Execute for Request {}
impl Execute for RequestStream {}
impl Execute for GetCommissionDefE{}

//pub trait ValidResponse {}
impl ValidResponse for LoginResponse{}
impl ValidResponse for GetCommissionDefResponse{}
impl ValidResponse for GetCurrentUserDataResponse{}
impl ValidResponse for GetCommissionDefE{}
impl ValidResponse for ErrorResponse{}