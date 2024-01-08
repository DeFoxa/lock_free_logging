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

struct Logger<'a> {
    formats: HashMap<LogMsg<'a>, String>,
}
impl<'a> Logger<'a> {
    fn log(&self, message: LogMsg) {
        // let format_str = self.formats.get(&message.warning).unwrap(); // old version before enum
        // switch, add pattern matching for log formatting
    }
    fn log_from_deserialized_generic<T>(&self, message: &T) {
        // Not sure if this will work from a performance standpoint, may have to seperate streams and call specific log method on msg relative to deserialized stream message type, have to think about this more and test.
        //
        // TODO: Takes generic type representing different variations of deserialized stream data,
        // matches data elements to log variables and calls logs with correct methods based on msg
        // data components. Adding TODO for implementation tomorrow morning.
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
//NOTE: these fields on LogMsg are testing examples, need to change inputs before adding to trade
//execution platform
enum LogMsg<'a> {
    Event(EventTypes<'a>),
    Warning {
        warning_message: &'a str,
    },
    Info {
        timestamp: &'a str,
        details: &'a str,
    },
    Error {
        error_code: i32,
        error_message: &'a str,
    },
}
impl<'a> LogMsg<'a> {
    fn format(&self) -> String {
        match self {
            LogMsg::Error {
                error_code,
                error_message,
            } => {
                format!("Error {}: {}", error_code, error_message)
            }
            LogMsg::Warning { warning_message } => {
                format!("Warning:  {}", warning_message)
            }
            LogMsg::Info { timestamp, details } => {
                format!("[{}] Info: {}", timestamp, details)
            }
            LogMsg::Event(_) => {
                todo!();
                // match to eventtypes
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum EventTypes<'a> {
    MarketOrderBookUpdate {
        symbol: &'a str,
        bids: Vec<[String; 2]>,
        asks: Vec<[String; 2]>,
        event_timestamp: &'a str,
    },
    MarketTradesUpdate {
        symbol: &'a str,
        side: &'a str,
        qty: &'a str,
        fill_price: &'a str,
        timestamp: &'a str,
    },
    LimitFill,
    MarketFill,
    PositionStatus,
}

// method block on LogMsg to instantiate each msg type, then matching function to take data from
// WS stream or trade engine and log based on the input formatting -> LogMsg enum -> logger. Or add output from trade engine methods that directly ouputst he LogMsg type and sends to logger
