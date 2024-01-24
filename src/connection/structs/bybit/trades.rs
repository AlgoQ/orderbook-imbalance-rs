use serde::{Deserialize, Serialize};
use super::base::BaseWs;

#[derive(Serialize, Deserialize)]
pub struct TradesWs {
    #[serde(flatten)]
    pub base: BaseWs,
    pub data: Vec<TradesDataWs>,
}

#[derive(Serialize, Deserialize)]
pub struct TradesDataWs {
    #[serde(rename = "T")]
    pub timestamp: i64,
    #[serde(rename = "s")]
    symbol_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "v")]
    pub amount: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "BT")]
    block_trade: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradesDataItem {
    pub timestamp: i64,
    pub symbol: String,
    pub trade_id: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub orderbook_imbalance_1: String,
    pub orderbook_imbalance_5: String,
    pub orderbook_imbalance_10: String,
    pub orderbook_imbalance_25: String,
    pub orderbook_imbalance_50: String
}

impl TradesDataItem {
    pub fn new(trade_data: TradesDataWs, orderbook_imbalances: [f64; 5]) -> TradesDataItem {
        // orderbook_imbalances to seperate once
        TradesDataItem {
            timestamp: trade_data.timestamp,
            symbol: trade_data.symbol_id,
            trade_id: trade_data.trade_id,
            side: trade_data.side.to_lowercase(),
            amount: trade_data.amount,
            price: trade_data.price,
            orderbook_imbalance_1: orderbook_imbalances[0].to_string(),
            orderbook_imbalance_5: orderbook_imbalances[1].to_string(),
            orderbook_imbalance_10: orderbook_imbalances[2].to_string(),
            orderbook_imbalance_25: orderbook_imbalances[3].to_string(),
            orderbook_imbalance_50: orderbook_imbalances[4].to_string()
        }
    }
}