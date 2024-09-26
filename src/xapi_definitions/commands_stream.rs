use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize, Serialize)]
//#[serde(rename_all = "lowercase", tag = "command", content = "arguments")]
//#[serde(rename_all = "lowercase", tag = "command")]
#[serde(rename_all="camelCase", tag = "command")]
pub enum RequestStream {
    GetCandles(GetCandles),
    GetBalance(GetBalance),
    GetKeepAlive(GetKeepAlive),
    GetTickPrices(GetTickPrices),
    GetTrades(GetTrades),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase", tag = "command")]
pub enum ResponseStream {
    Candle(GetResponse<GetCandlesResponse>),
    Balance(GetResponse<GetBalanceResponse>),
    KeepAlive(GetResponse<GetKeepAliveResponse>),
    TickPrices(GetResponse<GetTickPricesResponse>),
    Trade(GetResponse<GetTradesReponse>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse <T: Serialize> {
    pub data: T, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCandles {
    pub stream_session_id: String,
	pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCandlesResponse {
    pub close: f32,
    pub ctm: i64,
    pub ctm_string: String,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub quote_id: i32,
    pub symbol: String,
    pub vol: f32, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalance {
    pub stream_session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub balance: f32,
    pub credit: f32,
    pub equity: f32,
    pub margin: f32,
    pub margin_free: f32,
    pub margin_level: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKeepAlive {
    pub stream_session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKeepAliveResponse {
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickPrices {
    pub stream_session_id: String,
    pub symbol: String,
    pub min_arrival_time: Option<i32>,
    pub max_level: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickPricesResponse {
    pub ask: f32,
    pub ask_volume: i32,
    pub bid: f32,
    pub bid_volume: i32,
    pub high: f32,
    pub level: i32,
    pub low: f32,
    pub quote_id: i32,
    pub spread_raw: f32,
    pub spread_table: f32,
    pub symbol: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrades {
    pub stream_session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTradesReponse {
   pub  close_price: f32,
   pub  close_time: Option<i64>,
   pub  closed: bool,
   pub  cmd: Cmd,
   pub  comment: String,
   pub  commission: f32,
   #[serde(rename = "customComment")]
   pub  custom_comment: Option<String>,
   pub  digits: u16,
   pub  expiration: Option<u32>,
   pub  margin_rate: f32,
   pub  offset: u16,
   pub  open_price: f32,
   pub  open_time: i64,
   pub  order: u32,
   pub  order2: u32,
   pub  position: u32,
   pub  profit: Option<f32>,
   pub  sl: f32,
   pub  state: State,
   pub  storage: f32,
   pub  symbol: String,
   pub  tp: f32,
   pub  r#type: r#Type,
   pub  volume: f32, 
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum Cmd {
    Buy = 0,
    Sell = 1,
    BuyLimit = 2,
    SellLimit = 3,
    BuyStop = 4,
    SellStop = 5,
    Balance = 6,
    Credit = 7,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum State {
    Modified,
    Deleted,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum r#Type {
    Open = 0,
    Pending = 1,
    Close = 2,
    Modify = 3,
    Delete = 4,
}