pub mod connection;
use crate::connection::bybit;

use crate::connection::utils::enums::BybitCategory;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // depth 1, 5, 10, 25, 50
    // let imbalance_ranges = Arc::new(Mutex::new(HashMap::<String, f64>::new()));
    let imbalance_ranges = Arc::new(Mutex::new(HashMap::<String, [f64;5]>::new()));

    let symbols = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];

    for symbol in &symbols {
        imbalance_ranges.lock().unwrap().insert(symbol.clone(), [-1.0, -1.0, -1.0, -1.0, -1.0]);
    }
    

    let orderbooks_shared = Arc::clone(&imbalance_ranges);
    let trades_shared = Arc::clone(&imbalance_ranges);
    
    let watch_orderbooks_task = tokio::spawn(bybit::ws::watch_orderbooks(BybitCategory::Linear, symbols.clone(), None, orderbooks_shared));
    let watch_trades_task = tokio::spawn(bybit::ws::watch_trades(BybitCategory::Linear, symbols.clone(), trades_shared));

    if let Err(err) = tokio::try_join!(watch_orderbooks_task, watch_trades_task) {
        eprintln!("Error joining tasks: {:?}", err);
    }
}