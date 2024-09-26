use serde::{Deserialize, Serialize};
use serde_repr::*;

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

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum r#Type {
    Open = 0,
    Pending = 1,
    Close = 2,
    Modify = 3,
    Delete = 4,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum State {
    Modified,
    Deleted,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RequestStatus {
    Error = 0,
    Pending = 1,
    Accepted = 3,
    Rejected = 4,
}
