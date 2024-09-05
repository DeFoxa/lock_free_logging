use futures::executor::block_on;
use lib::types::*;
use lib::{enum_logger::*, raw_func_logger::*};
use std::sync::Arc;
use std::time::{Duration, Instant};
//tmp

fn run_benchmark<F>(name: &str, iterations: u32, mut f: F)
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
    let iterations = 1000000;

    let mut raw_func_logger = RawFuncLogger::new();

    run_benchmark("RawFunc Bench", iterations, || {
        let start = Instant::now();
        let _ = block_on(raw_func_logger.log(Arc::new(LogMsg::Warning {
            warning_message: ".",
        })));
        start.elapsed()
    });

    let mut enum_logger = EnumLogger::new();

    run_benchmark("LogMsg enum Bench", iterations, || {
        let start = Instant::now();
        let _ = block_on(enum_logger.log(LogMsg::Warning {
            warning_message: ".",
        }));
        start.elapsed()
    });

    let mut alternate_enum_logger = EnumLogger::new();

    run_benchmark("ExampleOB enum Bench", iterations, || {
        let example_ob = ExampleOB {
            symbol: 1,
            bids: [[50000, 100], [49900, 200], [49800, 150]].to_vec(),
            asks: [[50100, 120], [50200, 180], [50300, 90]].to_vec(),
            timestamp: 1629382400000,
        };

        let start = Instant::now();
        let _ = block_on(alternate_enum_logger.log(example_ob));
        start.elapsed()
    });
}
