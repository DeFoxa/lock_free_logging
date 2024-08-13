use chrono::Utc;
use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use lib::enum_logger::enum_logger;
use lib::example_types::*;
use lib::raw_func_logger::*;

//NOTE: Bench prints
//
// Bench Logging the LogMsg with enum defined in logging function
async fn log_bench() {
    let _ = async_logging_thread().await;
}

async fn enum_log_bench() {
    let ob_update: ExampleOB = ExampleOB {
        symbol: "BTCUSDT".to_string(),
        bids: vec![[100, 75]],
        asks: vec![[101, 255]],
        timestamp: Utc::now().timestamp_millis(),
    };
    enum_logger(ob_update).await;
}

// Owned message passed to Logger
async fn owned_log_msg_passed_to_logger_bench() {
    let data = OwnedLogMsg::Event(OwnedEventType::MarketOrderBookUpdate {
        symbol: "BTCUSDT".to_string(),
        bids: vec![[100, 75]],
        asks: vec![[101, 255]],
        event_timestamp: Utc::now().timestamp_millis(),
    });

    let _ = async_logger::<OwnedLogMsg>(data.into()).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Message formatted in log function", |b| {
        b.to_async(FuturesExecutor).iter(|| log_bench());
    });
    c.bench_function("OwnedLogmsg passed to logger", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| owned_log_msg_passed_to_logger_bench());
    });
    c.bench_function("Simple Enum", |b| {
        b.to_async(FuturesExecutor).iter(|| enum_log_bench());
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
