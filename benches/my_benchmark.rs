use chrono::Utc;
use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use lib::example_types::*;
use lib::logger::*;

//NOTE: Bench prints
//
// Bench Logging the LogMsg with enum defined in logging function
async fn log_bench() {
    let _ = async_logging_thread().await;
}

// Owned message passed to Logger
async fn owned_log_msg_passed_to_logger_bench() {
    let data = OwnedLogMsg::Event(OwnedEventType::MarketTradeData {
        symbol: "BTCUSDT".to_string(),
        side: "buy".to_string(),
        qty: 1,
        fill_price: 46030.50,
        timestamp: Utc::now().timestamp_millis(),
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
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
