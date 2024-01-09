#![allow(warnings)]
use crate::lib::*;
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone, Utc};
// use criterion::{criterion_group, criterion_main, Criterion};
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;

mod lib;

//NOTE: Lock Free concurrent logging with compile time Log messages instead of passing Strings or some other type
// around the threads for formatting/logging.

#[tokio::main]
async fn main() -> Result<()> {
    async_logging_thread().await;

    let data = OwnedEventType::MarketTradeData {
        symbol: ("BTCUSDT".to_string()),
        side: ("BUY".to_string()),
        qty: (1000),
        fill_price: (29000.69),
        timestamp: (Utc::now().timestamp_millis().to_string()),
    };

    //NOTE Example directly to log_thread
    let message = Arc::new(OwnedLogMsg::Event(data));
    async_logging_thread_with_message::<OwnedLogMsg>(message).await;

    //NOTE example processed from struct type to LogMsg -> thread with usage_example function
    let trade_data = ExampleTradeStream {
        symbol: "BTCUSDT".to_string(),
        side: "buy".to_string(),
        qty: 1,
        price: 46030.50,
        timestamp: Utc::now().timestamp_millis().to_string(),
    };

    usage_example::<ExampleTradeStream>(trade_data.into()).await;

    Ok(())
}
