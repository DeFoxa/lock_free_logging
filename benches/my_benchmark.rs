use chrono::Utc;
use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use lock_free_logging::*;

// Bench formatting struct as LogMsg
async fn to_log_message_bench() {
    let _data = ExampleTradeStream {
        symbol: "BTCUSDT".to_string(),
        side: "buy".to_string(),
        qty: 2,
        price: 46030.50,
        timestamp: Utc::now().timestamp_millis().to_string(),
    };
    usage_example::<ExampleTradeStream>(_data.into())
        .await
        .unwrap();
}

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
        timestamp: Utc::now().timestamp_millis().to_string(),
    });
    // let message = Arc::new(LogMsg::Event(data));
    let _ = async_logging_thread_with_message::<OwnedLogMsg>(data.into()).await;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("struct to LogMsg", |b| {
        b.to_async(FuturesExecutor).iter(|| to_log_message_bench());
    });
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

//
// fn bench_function(c: &mut Criterion) {
//     let rt = Runtime::new().unwrap();
//     c.bench_function("async_logging_bench", |b| {
//         b.to_async(&rt).iter(|| async {
//             let _data = ExampleTradeStream {
//                 symbol: "BTCUSDT".to_string(),
//                 side: "buy".to_string(),
//                 qty: 1,
//                 price: 46030.50,
//                 timestamp: Utc::now().timestamp_millis().to_string(),
//             };
//             let _ = usage_example::<ExampleTradeStream>(_data.into()).await;
//
//             let data = OwnedEventType::MarketTradeData {
//                 symbol: ("BTCUSDT".to_string()),
//                 side: ("BUY".to_string()),
//                 qty: (1000),
//                 fill_price: (29000.69),
//                 timestamp: (Utc::now().timestamp_millis().to_string()),
//             };
//
//             //NOTE Example directly to log_thread
//             let message = Arc::new(OwnedLogMsg::Event(data));
//             let test = async_logging_thread_with_message::<OwnedLogMsg>(message).await;
//
//             // async_logging_thread_with_message(Arc::new(_data)).await;
//         });
//     });
// }
// fn test() -> Result<()> {
//
// }
