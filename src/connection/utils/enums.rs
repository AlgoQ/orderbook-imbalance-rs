use serde::{Deserialize, Serialize};
use std::string::ToString;

pub enum RestMethod {
    GET,
    POST,
    PUT,
    DELETE
}

// Bybit

pub enum BybitCategory {
    Spot,
    Linear,
    Inverse,
    Option
}

impl ToString for BybitCategory {
    fn to_string(&self) -> String {
        match self {
            BybitCategory::Spot => "spot".to_string(),
            BybitCategory::Linear => "linear".to_string(),
            BybitCategory::Inverse => "inverse".to_string(),
            BybitCategory::Option => "option".to_string(),
        }
    }
}

pub enum BybitStatus {
    PreLaunch,
    Trading,
    Settling,
    Delivering,
    Closed
}

impl ToString for BybitStatus {
    fn to_string(&self) -> String {
        match self {
            BybitStatus::PreLaunch => "PreLaunch".to_string(),
            BybitStatus::Trading => "Trading".to_string(),
            BybitStatus::Settling => "Settling".to_string(),
            BybitStatus::Delivering => "Delivering".to_string(),
            BybitStatus::Closed => "Closed".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DirectionPriceChange {
    PlusTick, // Price rise
    ZeroPlusTick, // Trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
    MinusTick, // Price drop
    ZeroMinusTick, // trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
}