#![allow(warnings)]
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone};
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::collections::HashMap;
// use tokio::thread::spawn;;
use std::thread;

#[tokio::main]
async fn main() -> Result<()> {
    let async_logging = async_logging_thread().await;
    Ok(())
}

async fn async_logging_thread() -> Result<()> {
    let (mut sx, mut rx) = create::<RawFunc>();
    let guard = thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());
        match rx.recv() {
            Ok(msg) => {
                msg.invoke();
            }
            Err(e) => {
                panic!("error yada ");
            }
        }
    });
    let date = Local::now();
    let log_msg = format!(
        "ts: {}, volue: {}, price: {}, flag: {}",
        date.format("%Y-%m-%d %H:%M:%S"),
        100.01,
        20000,
        true
    );
    let log = sx.send(RawFunc::new(move || {
        println!(
            "ts: {}, volue: {}, price: {}, flag: {}",
            date.format("%Y-%m-%d %H:%M:%S"),
            100.01,
            20000,
            true
        );
    }));
    // .unwrap();

    Ok(())
}

struct Logger {
    formats: HashMap<LogMsg, String>,
}
impl Logger {
    fn send(&self, message: RawFunc) {
        let format_str = self.formats.get(&message.data).unwrap();
    }
}

struct RawFunc {
    data: Box<dyn Fn() + Send + 'static>,
}

impl RawFunc {
    fn new<T>(data: T) -> RawFunc
    where
        T: Fn() + Send + 'static,
    {
        return RawFunc {
            data: Box::new(data),
        };
    }
    fn invoke(self) {
        (self.data)()
    }
}
#[derive(Eq, PartialEq, Hash)]
enum LogMsg {
    Event(EventTypes),
    Warning,
    Info,
    Error,
}

#[derive(Eq, PartialEq, Hash)]
enum EventTypes {
    OrderBookUpdate,
    TradeUpdate,
    LimitFill,
}
