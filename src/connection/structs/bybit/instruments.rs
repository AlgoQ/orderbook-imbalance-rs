use serde::{Deserialize, Serialize};
use super::base::BaseRest;

#[derive(Serialize, Deserialize)]
pub struct InstrumentsRest {
    #[serde(flatten)]
    base: BaseRest,
    result: InstrumentsDataRest,
}

#[derive(Serialize, Deserialize)]
struct InstrumentsDataRest {
    category: String,
    list: Vec<InstrumentInfo>
}

#[derive(Serialize, Deserialize)]
struct InstrumentInfo {
    symbol: String,
    #[serde(rename = "baseCoin")]
    base_coin: String,
    #[serde(rename = "quoteCoin")]
    quote_coin: String,
    innovation: String,
    status: String,
    #[serde(rename = "marginTrading")]
    margin_trading: String,
    #[serde(rename = "lotSizeFilter")]
    lot_size_filter: LotSizeFilter,
    #[serde(rename = "priceFilter")]
    price_filter: PriceFilter,
}

#[derive(Serialize, Deserialize)]
struct LotSizeFilter {
    #[serde(rename = "basePrecision")]
    base_precision: String,
    #[serde(rename = "quotePrecision")]
    quote_precision: String,
    #[serde(rename = "minOrderQty")]
    min_order_qty: String,
    #[serde(rename = "maxOrderQty")]
    max_order_qty: String,
    #[serde(rename = "minOrderAmt")]
    min_order_amt: String,
    #[serde(rename = "maxOrderAmt")]
    max_order_amt: String,
}

#[derive(Serialize, Deserialize)]
struct PriceFilter {
    #[serde(rename = "tickSize")]
    tick_size: String,
}