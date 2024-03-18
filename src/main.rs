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

//NOTE: A few Examples of Lock Free concurrent logging for  usage later: includes types LogMsg/OwnedLogMsg types and Fn Pointer for async_thread closure, includes Struct type into LogMsg. Benchmarking with Criterion set up. Think there are a few things that can be optimized before implementing, but this provides the general framework of the logger. Bench with custom serializer against to_log_msg methods

//NOTE: logging types should be further compile time optimized: current implementation is for
//example testing

//NOTE: A few different variations of the logger in lib.rs, with owned data, with borrowed, with Formattable trait (and without), and with ToLogMsg::to_log_msg(): formattable trait added to pass types around as generics with attribute "Formattable" implemented on OwnedLogMsg (not implemented on LogMsg add to LogMsg for actual implementation), to_log_msg() trait/method for taking deserialized stream data or warn/error struct and convert to a log msg. Use as example code but update for consistency relative to implementation requirements.

#[tokio::main]
async fn main() -> Result<()> {
    async_logging_thread().await;

    let data = OwnedEventType::MarketTradeData {
        symbol: ("BTCUSDT".to_string()),
        side: ("BUY".to_string()),
        qty: (1000),
        fill_price: (295330.55),
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
