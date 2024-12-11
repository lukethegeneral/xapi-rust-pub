use super::commands_common::{Cmd, Type, RequestStatus};

use serde::{Deserialize, Serialize};

pub trait ValidResponse {}

impl ValidResponse for LoginResponse{}
impl ValidResponse for GetCommissionDefResponse{}
impl ValidResponse for GetCurrentUserDataResponse{}
impl ValidResponse for ErrorResponse{}
impl ValidResponse for TradeTransactionResponse{}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
pub enum Request {
    Login(LoginRequest),
    Logout(LogoutRequest),
    GetMarginTrade(GetMarginTradeRequest),
    GetChartLast(GetChartLastRequest),
    GetSymbol(GetSymbol),
    GetCommissionDef(GetCommissionDef),
    GetCurrentUserData(GetCurrentUserData),
    TradeTransaction(TradeTransaction),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Response<T:ValidResponse + Serialize> {
    Login(LoginResponse),
    Data(GetResponse<T>),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse <T: Serialize> {
    pub status: bool,
    pub return_data: T, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String,
    pub app_id: String,
    pub app_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub status: bool,
    pub stream_session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub status: bool,
    pub error_code: String,
    pub error_descr: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    pub command: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMarginTradeRequest {
    pub symbol: String,
    pub volume: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetChartLastRequest {
    pub period: u32,
    pub start: u32,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSymbol {
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCommissionDef {
    pub symbol: String,
    pub volume: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionDefResponse {
    pub commission: f32,
    pub rate_of_exchange: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCurrentUserData {
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentUserDataResponse {
    pub company_unit: i32,
    pub currency: String,
    pub group: String,
    pub ib_account: bool,
    pub leverage: i32,
    pub leverage_multiplier: f32,
    pub spread_type: Option<String>,
    pub trailing_stop: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeTransaction {
    pub trade_trans_info: TradeTransInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeTransInfo {
   pub  cmd: Cmd,
   #[serde(rename = "customComment")]
   pub  custom_comment: Option<String>,
   pub  expiration: u32,
   pub  offset: u16,
   pub  order: u32,
   pub  price: f32,
   pub  sl: f32,
   pub  symbol: String,
   pub  tp: f32,
   pub  r#type: r#Type,
   pub  volume: f32, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeTransactionResponse {
    pub order: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeTransactionStatus {
    pub order: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeTransactionStatusResponse {
    pub ask: f32,
    pub bid: f32,
    #[serde(rename = "customComment")]
    pub custom_comment: Option<String>,
    pub message: Option<String>,
    pub order: u32,
    pub request_status: RequestStatus,
}