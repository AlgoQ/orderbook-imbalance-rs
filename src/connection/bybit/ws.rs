use std::cmp;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures_util::{stream::StreamExt, SinkExt};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::connection::structs::bybit::orderbook::{OrderbookWs, FinalOrderbook};
use crate::connection::structs::bybit::trades::{TradesWs, TradesDataItem};
use crate::connection::utils::enums::BybitCategory;

use crate::connection::postgresql::insert_many::insert_many_trades;

use dotenv::dotenv;
use std::env;

const BASE_URL:&str = "wss://stream.bybit.com/v5/public/";

fn parse_url(uri: &str) -> Result<Url, url::ParseError> {
    match Url::parse(uri) {
        Ok(url) => Ok(url),
        Err(err) => Err(err),
    }
}

pub async fn watch_orderbooks(category:BybitCategory, symbols:Vec<String>, depth:Option<u16>, imbalance_ranges:Arc<Mutex<HashMap::<String, [f64;5]>>>) {
    // TODO: add `handle_orderbooks()` to input params
    fn round_to_2_digits(imbalances: [f64;5]) -> [f64;5] {
        let mut result = imbalances;
        for i in 0..imbalances.len() {
            let value = imbalances[i];
            let rounded_value = (value * 100.0).round() / 100.0;
            result[i] = rounded_value;
        };
        result
    }
    
    fn deserialize_raw_orderbook(orderbook: &mut FinalOrderbook, txt: String) {
        if txt.contains("topic") {
            let text_str: &str = &txt;
            let serialized_txt_resp: OrderbookWs = serde_json::from_str(text_str).unwrap();
            if txt.contains("snapshot") {
                orderbook.update_snapshot(&serialized_txt_resp);
            } else {
                orderbook.update_delta(&serialized_txt_resp);
            }
        }
    }

    let uri = BASE_URL.to_owned() + &category.to_string();

    let parse_url_result = parse_url(&uri);
    
    let ws_stream = match parse_url_result {
        Ok(url) => {
            let (ws_stream, _) = connect_async(url)
                .await
                .expect("Failed to connect");
            ws_stream
        }
        Err(err) => {
            eprintln!("Error parsing URL: {}", err);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    let mut streams = Vec::new();

    let mut orderbooks: HashMap<String, FinalOrderbook> = HashMap::new();
    
    for symbol in symbols {
        let mut stream = format!("orderbook.50.{symbol}");
        if let Some(mut depth_unwrapped) = depth {
            match category {
                BybitCategory::Linear | BybitCategory::Inverse => {
                    depth_unwrapped = cmp::min(depth_unwrapped, 500);
                },
                BybitCategory::Spot => {
                    depth_unwrapped = cmp::min(depth_unwrapped, 200);
                },
                BybitCategory::Option => {
                    depth_unwrapped = cmp::min(depth_unwrapped, 100);
                }
            }
            stream = format!("orderbook.{depth_unwrapped}.{symbol}");
        }
        streams.push(stream.clone());
        orderbooks.insert(stream, FinalOrderbook::new());
    }

    match serde_json::to_string(&serde_json::json!({
        "op": "subscribe",
        "args": streams
    })) {
        Ok(message_text) => {
            match write.send(Message::text(message_text)).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error sending WebSocket message: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error serializing JSON: {}", err);
        }
    }

    while let Some(message) = read.next().await {
        match message { // TODO: Handle ping/pong
            Ok(Message::Text(txt)) => if txt.contains("topic") {
                let split_parts: Vec<&str> = txt.split('"').collect();
                let stream = split_parts.get(3).unwrap().to_string();
                if let Some(orderbook) = orderbooks.get_mut(&stream) {
                    deserialize_raw_orderbook(orderbook, txt);
                    let imbalances = orderbook.calculate_imbalances();
                    let new_imbalance_ranges = round_to_2_digits(imbalances);
                    let mut ranges = imbalance_ranges.lock().unwrap();
                    let symbol = stream.split('.').last().unwrap();
                    if let Some(imbalance_ranges) = ranges.get_mut(symbol) {
                        if new_imbalance_ranges != *imbalance_ranges {
                            *imbalance_ranges = new_imbalance_ranges;
                        }
                    }
                }
            }
            _ => (),
        };        
    }
}

pub async fn watch_trades(category: BybitCategory, symbols: Vec<String>, imbalance_ranges:Arc<Mutex<HashMap::<String, [f64;5]>>>) {
    // TODO: add `handle_trades()` to input params
    async fn deserialize_raw_trades(txt: String, pool:PgPool, imbalance_ranges:[f64;5]) {
        if txt.contains("topic") {
            let text_str: &str = &txt;
            let serialized_txt_resp: TradesWs = serde_json::from_str(text_str).unwrap();
            let mut trades_data: Vec<TradesDataItem> = Vec::new();

            for trade_data in serialized_txt_resp.data {
                let trade_sql = TradesDataItem::new(trade_data, imbalance_ranges);
                trades_data.push(trade_sql);
            }
            let _ = insert_many_trades(&pool, trades_data).await;
        }
    }

    let uri = BASE_URL.to_owned() + &category.to_string();
    let parse_url_result = parse_url(&uri);

    let ws_stream = match parse_url_result {
        Ok(url) => {
            let (ws_stream, _) = connect_async(url)
                .await
                .expect("Failed to connect");
            ws_stream
        }
        Err(err) => {
            eprintln!("Error parsing URL: {}", err);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    let mut streams = Vec::new();

    for symbol in symbols {
        let stream = format!("publicTrade.{symbol}");
        streams.push(stream);
    }

    match serde_json::to_string(&serde_json::json!({
        "op": "subscribe",
        "args": streams
    })) {
        Ok(message_text) => {
            match write.send(Message::text(message_text)).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error sending WebSocket message: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error serializing JSON: {}", err);
        }
    }

    dotenv().expect("Failed to load .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let pool = match PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await {
        Ok(pool) => pool,
        Err(err) => panic!("Failed to create Postgres pool: {}", err),
    };


    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(txt)) => {
                if txt.contains("topic") {
                    let split_parts: Vec<&str> = txt.split('"').collect();
                    let stream = split_parts.get(3).unwrap().to_string();
                    let symbol = stream.split('.').last().unwrap();
                    let imbalance_ranges = {
                        let ranges = imbalance_ranges.lock().unwrap();
                        ranges.get(symbol).unwrap().clone()
                    };
                    let _ = deserialize_raw_trades(txt, pool.clone(), imbalance_ranges).await;
                }
            }
            _ => (),
        };
    }
}