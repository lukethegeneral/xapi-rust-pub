use serde::{Deserialize, Serialize};

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
    TickPrices(GetResponse<GetTickPricesResponse>),
    Balance(GetResponse<GetBalanceResponse>),
    KeepAlive(GetResponse<GetKeepAliveResponse>),
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
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrades {
    pub stream_session_id: String,
}