use chrono::Utc;
use criterion::{criterion_group, criterion_main, Criterion};
use lock_free_logging::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn bench_function(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("async_logging_bench", |b| {
        b.to_async(&rt).iter(|| async {
            let _data = ExampleTradeStream {
                symbol: "BTCUSDT".to_string(),
                side: "buy".to_string(),
                qty: 1,
                price: 46030.50,
                timestamp: Utc::now().timestamp_millis().to_string(),
            };
            let _ = usage_example::<ExampleTradeStream>(_data.into()).await;

            let data = OwnedEventType::MarketTradeData {
                symbol: ("BTCUSDT".to_string()),
                side: ("BUY".to_string()),
                qty: (1000),
                fill_price: (29000.69),
                timestamp: (Utc::now().timestamp_millis().to_string()),
            };

            //NOTE Example directly to log_thread
            let message = Arc::new(OwnedLogMsg::Event(data));
            let test = async_logging_thread_with_message::<OwnedLogMsg>(message).await;

            // async_logging_thread_with_message(Arc::new(_data)).await;
        });
    });
}

criterion_group!(benches, /* benchmark_async_logging */ bench_function);

criterion_main!(benches);
