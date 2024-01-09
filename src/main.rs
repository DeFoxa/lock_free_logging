#![allow(warnings)]
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone, Utc};
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

//NOTE: Lock Free concurrent logging with compile time Log messages instead of passing Strings or some other type
// around the threads for formatting/logging.
//NOTE:

#[tokio::main]
async fn main() -> Result<()> {
    async_logging_thread().await;

    let data = OwnedEventType::MarketTradeData {
        symbol: ("BTCUSDT".to_string()),
        side: ("BUY".to_string()),
        qty: (1000),
        fill_price: (29000.69),
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

async fn async_logging_thread() -> Result<()> {
    let (mut sx, mut rx) = create::<RawFunc>();
    let guard = thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());
        while let Ok(raw_func) = rx.recv() {
            raw_func.invoke();
        }
    });

    /* Implementation Example */
    let logger = Logger::new();
    let log_message = LogMsg::Warning {
        warning_message: "testing_message",
    };
    let raw_func = RawFunc::new(move || logger.log(log_message.clone()));
    sx.send(raw_func);

    Ok(())
}

async fn async_logging_thread_with_message<G>(log_message: Arc<G>) -> Result<()>
where
    G: Clone + Send + Sync + Formattable + 'static,
{
    let (mut sx, mut rx) = create::<RawFunc>();
    let guard = thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());
        while let Ok(raw_func) = rx.recv() {
            raw_func.invoke();
        }
    });

    let logger = OwnedDataLogger::new();
    let logger_context = LoggerWithContext::new(logger, log_message);
    let raw_func = RawFunc::new(move || {
        logger_context.log_with_context();
    });

    sx.send(raw_func);

    Ok(())
}

async fn usage_example<G: Clone + Send + Sync + ToLogMsg + 'static>(message: G) -> Result<()> {
    let log = message.to_log_msg();
    async_logging_thread_with_message::<OwnedLogMsg>(Arc::new(log.clone())).await;
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
    fn log_with_arc(&self, message: &Arc<LogMsg>) {
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

// RawFunc: lock-free fn pointer
struct RawFunc {
    func: Box<dyn Fn() + Send + 'static>,
}

impl RawFunc {
    fn new<T>(data: T) -> RawFunc
    where
        T: Fn() + Send + 'static,
    {
        RawFunc {
            func: Box::new(data),
        }
    }
    fn invoke(self) {
        (self.func)()
    }
}

//
//NOTE: These fields on LogMsg are testing examples, need to change inputs to match real data
//fields before adding to trade execution platform
//
trait Formattable {
    fn formatting(&self) -> String;
}
// trait to implement on deserialized stream data structs, formats the struct fields into LogMsg to
// be passed to async_logging_thread
trait ToLogMsg {
    fn to_log_msg(self) -> OwnedLogMsg;
}

struct OwnedDataLogger {
    formats: HashMap<OwnedLogMsg, String>,
}
impl OwnedDataLogger {
    fn new() -> Self {
        Self {
            formats: HashMap::new(),
        }
    }
    fn log(&self, message: LogMsg) {
        let formatted_msg = message.format();
        println!("{:?}", formatted_msg);
    }
    fn log_with_arc<G: Clone + Formattable>(&self, message: &Arc<G>) {
        let formatted_msg = message.as_ref().formatting();
        println!("{:?}", formatted_msg);
    }
}

struct LoggerWithContext<G>
where
    G: Clone + Send + Sync + Formattable,
{
    logger: OwnedDataLogger,
    log_message: Arc<G>,
}
impl<G> LoggerWithContext<G>
where
    G: Clone + Send + Sync + Formattable,
{
    fn new(logger: OwnedDataLogger, log_message: Arc<G>) -> Self
    where
        G: Clone + Send + Sync,
    {
        LoggerWithContext {
            logger,
            log_message,
        }
    }
    fn log_with_context(&self) {
        // self.logger.log_with_arc(&Arc::clone(&self.log_message));
        let formatted = self.log_message.formatting();
        println!("{:?}", formatted);
    }
}

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
            LogMsg::Event(event_types) => {
                match event_types {
                    EventTypes::MarketOrderBookUpdate {
                        symbol,
                        bids,
                        asks,
                        event_timestamp,
                    } => format!(
                        "MarketOrderBookUpdate - symbol: {}, bids {:?}, asks {:?}, event_timestamp {}",
                        symbol, bids, asks, event_timestamp
                    ),
                    EventTypes::MarketTradesUpdate {
                        symbol,
                        side,
                        qty,
                        fill_price,
                        timestamp,
                    } => format!(
                        "MarketTradesUpdate - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}", 
                        symbol, side, qty, fill_price, timestamp),
                    EventTypes::AccountPartialLimitFill {
                        symbol,
                        side,
                        price,
                        size_filled,
                        size_unfilled,
                        timestamp,
                    } => format!(
                        "AccountPartialLimitFill - symbol: {}, side: {}, price: {}, size_filled: {}, size_unfilled: {}, timestamp: {}",
                        symbol, side, price, size_unfilled, size_filled, timestamp),
                    EventTypes::AccountLimitFill {
                        symbol,
                        side,
                        fill_price,
                        qty,
                        timestamp,
                    } => format!(
                        "AccountLimitFill - symbol: {}, side: {}, fill_price: {}, qty: {}, timestamp: {}", 
                        symbol, side, fill_price, qty, timestamp),
                    EventTypes::AccountMarketFill {
                        symbol,
                        side,
                        qty,
                        fill_price,
                        timestamp,
                    } => format!(
                        "AccountMarketFill - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}",
                        symbol, side, qty, fill_price, timestamp),
                    EventTypes::AccountPositionStatus {
                        symbol,
                        side,
                        pnl,
                        leverage,
                        fill_timestamp,
                        time_since_fill,
                    } => format!(
                        "AccountPositionStatus - symbol: {}, side: {}, pnl: {}, leverage: {}, fill_timestamp: {}, time_since_fill: {} ",
                        symbol, side, pnl, leverage, fill_timestamp, time_since_fill,
                    ),
                }

                // match to eventtypes
            }
        }
    }
}

//NOTE: Example implementation: these field(s) and field types will change based on Deserialized stream data
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
    AccountPartialLimitFill {
        symbol: &'a str,
        side: &'a str,
        price: &'a str,
        size_filled: &'a str,
        size_unfilled: &'a str,
        timestamp: &'a str,
    },
    AccountLimitFill {
        symbol: &'a str,
        side: &'a str,
        fill_price: &'a str,
        qty: &'a str,
        timestamp: &'a str,
    },
    AccountMarketFill {
        symbol: &'a str,
        side: &'a str,
        qty: &'a str,
        fill_price: &'a str,
        timestamp: &'a str,
    },
    AccountPositionStatus {
        symbol: &'a str,
        side: &'a str,
        pnl: &'a str,
        leverage: &'a str,
        fill_timestamp: &'a str,
        time_since_fill: &'a str,
    },
}
// below are examples of possible structs on to which fn to_log_msg will be impl.
// NOTE: future implementations of lock_free_logger will require structuring deserialized stream
// data and error messages to LogMsg enum. using owned types for simplicity
#[derive(Debug, Clone)]
struct ExampleTradeStream {
    symbol: String,
    side: String,
    qty: i32,
    price: f64,
    timestamp: String,
}
impl ToLogMsg for ExampleTradeStream {
    fn to_log_msg(self) -> OwnedLogMsg {
        OwnedLogMsg::Event(OwnedEventType::MarketTradeData {
            symbol: self.symbol,
            side: self.side,
            qty: self.qty,
            fill_price: self.price,
            timestamp: self.timestamp,
        })
    }
}
// Example of error message type to implement S
#[derive(Debug, Clone)]
struct ExampleErrorMsg {
    error_code: i32,
    error_message: String,
}
impl ToLogMsg for ExampleErrorMsg {
    fn to_log_msg(self) -> OwnedLogMsg {
        OwnedLogMsg::Error {
            error_code: self.error_code,
            error_message: self.error_message,
        }
    }
}

// TESTING
#[derive(Debug, PartialEq, Clone)]
enum OwnedLogMsg {
    Event(OwnedEventType),
    Warning {
        warning_message: String,
    },
    Error {
        error_code: i32,
        error_message: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
enum OwnedEventType {
    MarketTradeData {
        symbol: String,
        side: String,
        qty: i32,
        fill_price: f64,
        timestamp: String,
    },
}
impl OwnedLogMsg {
    fn format(&self) -> String {
        match self {
            OwnedLogMsg::Warning { warning_message } => format!("warning: {}", warning_message),
            OwnedLogMsg::Event(data) => match data {
                OwnedEventType::MarketTradeData {
                    symbol,
                    side,
                    qty,
                    fill_price,
                    timestamp,
                } => format!(
                    "TradeData - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}",
                    symbol, side, qty, fill_price, timestamp
                ),
            },
            OwnedLogMsg::Error {
                error_code,
                error_message,
            } => format!("Error code {}, Message {}", error_code, error_message),
        }
    }
}
impl Formattable for OwnedLogMsg {
    fn formatting(&self) -> String {
        match self {
            OwnedLogMsg::Warning { warning_message } => format!("warning: {}", warning_message),
            OwnedLogMsg::Event(data) => match data {
                OwnedEventType::MarketTradeData {
                    symbol,
                    side,
                    qty,
                    fill_price,
                    timestamp,
                } => format!(
                    "TradeData - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}",
                    symbol, side, qty, fill_price, timestamp
                ),
                // match to eventtypes
            },
            OwnedLogMsg::Error {
                error_code,
                error_message,
            } => format!("Error Code: {}, Message {}", error_code, error_message),
        }
    }
}

// method block on LogMsg to instantiate each msg type, then matching function to take data from
// WS stream or trade engine and log based on the input formatting -> LogMsg enum -> logger. Or add output from trade engine methods that directly ouputst he LogMsg type and sends to logger
