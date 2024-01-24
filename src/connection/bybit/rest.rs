use std::cmp;

use crate::connection::utils::rest::fetch;

use crate::connection::utils::enums::{RestMethod, BybitCategory, BybitStatus};

use crate::connection::structs::bybit::orderbook::OrderBookRest;
use crate::connection::structs::bybit::instruments::InstrumentsRest;

use crate::connection::utils::url::add_params_to_url;


const BASE_URL:&str = "https://api.bybit.com";

pub async fn fetch_orderbook(category:BybitCategory, symbol:String, mut limit:Option<u8>) 
    -> Result<OrderBookRest, Box<dyn std::error::Error>> {
    
    let mut url = BASE_URL.to_owned() + "/v5/market/orderbook";
    match category {
        BybitCategory::Spot | BybitCategory::Linear | BybitCategory::Inverse => {
            limit = cmp::min(limit, Some(200));
        },
        BybitCategory::Option => {
            limit = cmp::min(limit, Some(25));
        }
    }
    url = add_params_to_url(url, vec![
        ("category".to_string(), Some(category.to_string())),
        ("symbol".to_string(), Some(symbol)),
        ("limit".to_string(), limit.map(|num| num.to_string()))
    ]);
    let orderbook: Result<OrderBookRest, _> = fetch(url, RestMethod::GET).await;
    orderbook
}

pub async fn fetch_instruments_info(category:BybitCategory, symbol:Option<String>, 
    status:Option<BybitStatus>, base_coin: Option<String>, mut limit: Option<u16>, cursor: Option<String>)
    -> Result<InstrumentsRest, Box<dyn std::error::Error>> {
    
    let mut url = BASE_URL.to_owned() + "/v5/market/instruments-info";
    limit = cmp::min(limit, Some(1000));
    url = add_params_to_url(url, vec![
        ("category".to_string(), Some(category.to_string())),
        ("symbol".to_string(), symbol),
        ("status".to_string(), status.map(|bybit_status| bybit_status.to_string())),
        ("baseCoin".to_string(), base_coin),
        ("limit".to_string(), limit.map(|num| num.to_string())),
        ("cursor".to_string(), cursor)
    ]);
    let instruments_info: Result<InstrumentsRest, _> = fetch(url, RestMethod::GET).await;
    instruments_info
}