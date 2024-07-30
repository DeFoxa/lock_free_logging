#![allow(warnings)]
use chrono::Utc;
use eyre::Result;
use lib::example_types::*;
use lib::logger::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    //NOTE: Commented out print statements from the logger::Logger and logger::OwnedDataLogger methods log and log_with_context.
    //NOTE:Running --bin main will not print to stdout unless print uncommented
    //
    let data = OwnedEventType::MarketTradeData {
        symbol: "BTCUSDT".to_string(),
        side: "BUY".to_string(),
        qty: 1000,
        fill_price: 295330.55,
        timestamp: Utc::now().timestamp_millis(),
    };

    //NOTE Example directly to log_thread
    let message = Arc::new(OwnedLogMsg::Event(data));

    async_logger::<OwnedLogMsg>(message).await;

    Ok(())
}
