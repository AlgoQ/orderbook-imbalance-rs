use serde::{Deserialize, Serialize};
use super::base::{BaseRest, BaseWs};

use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

// REST

#[derive(Serialize, Deserialize)]
pub struct OrderBookRest {
    #[serde(flatten)]
    base: BaseRest,
    result: OrderBookDataRest,
}

#[derive(Serialize, Deserialize)]
struct OrderBookDataRest {
    #[serde(rename = "s")]
    symbol_id: String,
    #[serde(rename = "b")]
    bids: Vec<[String; 2]>,
    #[serde(rename = "a")]
    asks: Vec<[String; 2]>,
    #[serde(rename = "ts")]
    timestamp: u64,
    #[serde(rename = "u")]
    update_id: u64,
}

// WS

#[derive(Serialize, Deserialize)]
pub struct OrderbookWs {
    #[serde(flatten)]
    pub base: BaseWs,
    pub data: OrderbookDataWs,
}

#[derive(Serialize, Deserialize)]
pub struct OrderbookDataWs {
    #[serde(rename = "s")]
    symbol_id: String,
    #[serde(rename = "b")]
    bids: Vec<[String; 2]>,
    #[serde(rename = "a")]
    asks: Vec<[String; 2]>,
    #[serde(rename = "u")]
    update_id: u64,
    #[serde(rename = "seq")]
    sequence: u64,
}

pub struct FinalOrderbook {
    pub timestamp: u64,
    bids: BTreeMap<OrderedFloat<f64>, f64>,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
    total_bids_quantity: f64,
    total_asks_quantity: f64,
}

impl FinalOrderbook {
    pub fn new() -> Self {
        FinalOrderbook {
            timestamp: 0,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            total_bids_quantity: 0.0,
            total_asks_quantity: 0.0,
        }
    }

    pub fn update_snapshot(&mut self, raw_orderbook: &OrderbookWs) {
        self.timestamp = raw_orderbook.base.timestamp;
        
        self.bids = raw_orderbook.data
        .bids
        .iter()
        .map(|[price, quantity]| (
            OrderedFloat(price.parse().unwrap()), 
            quantity.parse().unwrap())
        )
        .collect();
        
        self.asks = raw_orderbook.data
        .asks
        .iter()
        .map(|[price, quantity]| (
            OrderedFloat(price.parse().unwrap()),
            quantity.parse().unwrap()))
        .collect();

        self.update_totals();
    }

    pub fn update_delta(&mut self, raw_orderbook: &OrderbookWs) {
        self.timestamp = raw_orderbook.base.timestamp;

        for [price, quantity] in &raw_orderbook.data.bids {
            let price = OrderedFloat(price.parse().unwrap());
            let quantity = quantity.parse().unwrap();
            if quantity == 0.0 {
                self.bids.remove(&price);
            } else {
                self.bids.insert(price, quantity);
            }
        }

        for [price, quantity] in &raw_orderbook.data.asks {
            let price = OrderedFloat(price.parse().unwrap());
            let quantity = quantity.parse().unwrap();
            if quantity == 0.0 {
                self.asks.remove(&price);
            } else {
                self.asks.insert(price, quantity);
            }
        }

        self.update_totals();
    }

    fn update_totals(&mut self) {
        self.total_bids_quantity = self.bids.values().sum();
        self.total_asks_quantity = self.asks.values().sum();
    }

    pub fn calculate_imbalances(&self) -> [f64;5] {
        let mut result: [f64;5] = [-1.0; 5];
        let depths: [u8;5] = [1, 5, 10, 25, 50];
        
        for i in 0..depths.len() {
            let depth = depths[i];
            // TODO: Check if bid & asks order is fine
            let bid_quantity: f64 = self.bids.values().take(depth as usize).sum();
            let ask_quantity: f64 = self.asks.values().take(depth as usize).sum();
            let total_quantity: f64 = bid_quantity + ask_quantity;

            if total_quantity > 0.0 {
                result[i] = (bid_quantity / total_quantity).abs();
            } else {
                result[i] = -1.0;
            }
        };

        result
    }
}