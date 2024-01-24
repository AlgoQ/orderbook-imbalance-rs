use sqlx::{PgPool, types::BigDecimal, types::Uuid};
use crate::connection::structs::bybit::trades::TradesDataItem;
use std::str::FromStr;

pub async fn insert_many_trades(pool: &PgPool, trades_vec: Vec<TradesDataItem>) -> Result<(), sqlx::Error> {
    if trades_vec.is_empty() {
        return Ok(());
    }

    // TODO: Insert many
    for trade in trades_vec {
        let _ = sqlx::query!(
            "INSERT INTO orderbook_imbalance_trades (timestamp, symbol, trade_id, side, amount, price, orderbook_imbalance_1, orderbook_imbalance_5, orderbook_imbalance_10, orderbook_imbalance_25, orderbook_imbalance_50) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            trade.timestamp as i64,
            trade.symbol,
            Uuid::parse_str(&trade.trade_id).unwrap_or_default(),
            trade.side,
            BigDecimal::from_str(&trade.amount).map_err(|e| eprintln!("Error converting amount: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.price).map_err(|e| eprintln!("Error converting price: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.orderbook_imbalance_1).map_err(|e| eprintln!("Error converting orderbook_imbalance_1: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.orderbook_imbalance_5).map_err(|e| eprintln!("Error converting orderbook_imbalance_5: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.orderbook_imbalance_10).map_err(|e| eprintln!("Error converting orderbook_imbalance_5: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.orderbook_imbalance_25).map_err(|e| eprintln!("Error converting orderbook_imbalance_5: {:?}", e)).unwrap_or_default(),
            BigDecimal::from_str(&trade.orderbook_imbalance_50).map_err(|e| eprintln!("Error converting orderbook_imbalance_5: {:?}", e)).unwrap_or_default(),
        )
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            sqlx::Error::from(e)
        })?;
    };

    Ok(())
}