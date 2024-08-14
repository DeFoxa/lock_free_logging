use futures::executor::block_on;
use lib::example_types::*;
use lib::raw_func_logger::*;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn run_raw_func_benchmark<F>(name: &str, iterations: u32, mut f: F)
where
    F: FnMut() -> Duration,
{
    let mut total_duration = Duration::new(0, 0);

    for _ in 0..iterations {
        total_duration += f();
    }

    let avg_duration = total_duration / iterations;
    println!(
        "Bench_id: {} Avg time per iteration {:?}",
        name, avg_duration
    );
}

fn main() {
    let iterations = 10000000;

    let mut raw_func_logger = RawFuncLogger::new();

    run_raw_func_benchmark("RawFunc Bench", iterations, || {
        let start = Instant::now();
        let _ = block_on(raw_func_logger.log(Arc::new(OwnedLogMsg::Warning {
            warning_message: "test".to_string(),
        })));
        start.elapsed()
    });
}
