#![allow(warnings)]
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone, Utc};
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::collections::HashMap;
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
        while let Ok(raw_func) = rx.recv() {
            raw_func.invoke();
        }
    });

    let logger = Logger::new();

    let log_message = LogMsg::Warning {
        warning_message: "testing_message",
    };
    let raw_func = RawFunc::new(move || logger.log(log_message.clone()));
    sx.send(raw_func);

    Ok(())
}

struct Logger<'a> {
    formats: HashMap<LogMsg<'a>, String>,
}
impl<'a> Logger<'a> {
    fn new() -> Self {
        Self {
            formats: HashMap::new(),
        }
    }
    fn log(&self, message: LogMsg) {
        let formatted_msg = message.format();
        println!("{:?}", formatted_msg);
    }
    fn log_from_deserialized_generic<T>(&self, message: &T) {
        // Not sure if this will work from a performance standpoint, may have to seperate streams and call specific log method on msg relative to deserialized stream message type, have to think about this more and test.
        //
        // TODO: Takes generic type representing different variations of deserialized stream data,
        // matches data elements to log variables and calls logs with correct methods based on msg
        // data components.
    }
}

struct RawFunc {
    closure: Box<dyn Fn() + Send + 'static>,
}

impl RawFunc {
    fn new<T>(data: T) -> RawFunc
    where
        T: Fn() + Send + 'static,
    {
        return RawFunc {
            closure: Box::new(data),
        };
    }
    fn invoke(self) {
        (self.closure)()
    }
}
//NOTE: these fields on LogMsg are testing examples, need to change inputs before adding to trade
//execution platform
//
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum LogMsg<'a> {
    Event(EventTypes<'a>),
    Warning {
        warning_message: &'static str,
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

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
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
