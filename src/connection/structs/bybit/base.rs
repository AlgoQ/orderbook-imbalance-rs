use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BaseRest {
    #[serde(rename = "retCode")]
    ret_code: u8,
    #[serde(rename = "retMsg")]
    ret_msg: String,
    #[serde(rename = "time")]
    timestamp: u64
}

#[derive(Serialize, Deserialize)]
pub struct BaseWs {
    topic: String,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(rename = "type")]
    data_type: String,
}