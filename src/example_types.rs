#![allow(warnings)]
use crate::raw_func_logger::{Formattable, ToLogMsg};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum LogMsg<'a> {
    Event(NormalizedEventTypes<'a>),
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
    pub fn format(&self) -> String {
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
                    NormalizedEventTypes::MarketOrderBookUpdate {
                        symbol,
                        bids,
                        asks,
                        event_timestamp,
                    } => format!(
                        "MarketOrderBookUpdate - symbol: {}, bids {:?}, asks {:?}, event_timestamp {}",
                        symbol, bids, asks, event_timestamp
                    ),
                    NormalizedEventTypes::MarketTrade {
                        symbol,
                        side,
                        qty,
                        fill_price,
                        timestamp,
                    } => format!(
                        "MarketTrade - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}", 
                        symbol, side, qty, fill_price, timestamp),
                    NormalizedEventTypes::AccountPartialMakerFill {
                        symbol,
                        side,
                        price,
                        size_filled,
                        size_unfilled,
                        timestamp,
                    } => format!(
                        "AccountPartialMakerFill - symbol: {}, side: {}, price: {}, size_filled: {}, size_unfilled: {}, timestamp: {}",
                        symbol, side, price, size_unfilled, size_filled, timestamp),
                    NormalizedEventTypes::AccountMakerFill {
                        symbol,
                        side,
                        fill_price,
                        qty,
                        timestamp,
                    } => format!(
                        "AccountMakerFill - symbol: {}, side: {}, fill_price: {}, qty: {}, timestamp: {}", 
                        symbol, side, fill_price, qty, timestamp),
                    NormalizedEventTypes::AccountTakerFill {
                        symbol,
                        side,
                        qty,
                        fill_price,
                        timestamp,
                    } => format!(
                        "AccountTakerFill - symbol: {}, side: {}, qty: {}, fill_price: {}, timestamp: {}",
                        symbol, side, qty, fill_price, timestamp),
                    NormalizedEventTypes::AccountPositionStatus {
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

impl<'a> Formattable for LogMsg<'a> {
    fn formatting(&self) -> String {
        unimplemented!();
    }
}

//NOTE: Example implementation: these field(s) and field types will change based on Deserialized stream data
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum NormalizedEventTypes<'a> {
    MarketOrderBookUpdate {
        symbol: &'a str,
        bids: Vec<[i64; 2]>,
        asks: Vec<[i64; 2]>,
        event_timestamp: i64,
    },
    MarketTrade {
        symbol: &'a str,
        side: &'a str,
        qty: &'a str,
        fill_price: &'a str,
        timestamp: i64,
    },
    AccountPartialMakerFill {
        symbol: &'a str,
        side: &'a str,
        price: &'a str,
        size_filled: &'a str,
        size_unfilled: &'a str,
        timestamp: &'a str,
    },
    AccountMakerFill {
        symbol: &'a str,
        side: &'a str,
        fill_price: &'a str,
        qty: &'a str,
        timestamp: &'a str,
    },
    AccountTakerFill {
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
// below are examples of possible pub structs on to which fn to_log_msg will be impl.
// NOTE: future implementations of lock_free_logger will require pub structuring deserialized stream
// data and error messages to LogMsg enum. using owned types for simplicity
#[derive(Debug, Clone)]
pub struct ExampleOB {
    pub symbol: i32, // Setting as i32 instead of String, will never use a String allocation in log
    // hot path anyway, this will be more accurate
    pub bids: Vec<[i64; 2]>,
    pub asks: Vec<[i64; 2]>,
    pub timestamp: i64,
}
impl ToLogMsg for ExampleOB {
    fn to_log_msg(self) -> OwnedLogMsg {
        OwnedLogMsg::Event(OwnedEventType::MarketOrderBookUpdate {
            symbol: self.symbol.to_string(),
            bids: self.bids,
            asks: self.asks,
            event_timestamp: self.timestamp,
        })
    }
}
impl Formattable for ExampleOB {
    fn formatting(&self) -> String {
        format!(
            "OrderBook - Symbol: {}, Timestamp: {}\nBids: {}\nAsks: {}",
            self.symbol,
            self.timestamp,
            format_limits(&self.bids),
            format_limits(&self.asks)
        )
    }
}

fn format_limits(orders: &Vec<[i64; 2]>) -> String {
    orders
        .iter()
        .map(|&[price, amount]| format!("[Price: {}, Amount: {}]", price, amount))
        .collect::<Vec<String>>()
        .join(", ")
}

// Example of error message type to implement S
#[derive(Debug, Clone)]
pub struct ExampleErrorMsg {
    pub error_code: i32,
    pub error_message: String,
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
pub enum OwnedLogMsg {
    Event(OwnedEventType),
    Warning {
        warning_message: String,
    },
    Error {
        error_code: i32,
        error_message: String,
    },
}

//example
#[derive(Debug, PartialEq, Clone)]
pub enum OwnedEventType {
    MarketOrderBookUpdate {
        symbol: String,
        bids: Vec<[i64; 2]>,
        asks: Vec<[i64; 2]>,
        event_timestamp: i64,
    },
}
impl OwnedLogMsg {
    fn format(&self) -> String {
        match self {
            OwnedLogMsg::Warning { warning_message } => format!("warning: {}", warning_message),
            OwnedLogMsg::Event(data) => match data {
                OwnedEventType::MarketOrderBookUpdate {
                    symbol,
                    bids,
                    asks,
                    event_timestamp,
                } => format!(
                    "TradeData - symbol: {}, bids: {:?}, asks: {:?}, event_timestamp: {}",
                    symbol, bids, asks, event_timestamp
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
                OwnedEventType::MarketOrderBookUpdate {
                    symbol,
                    bids,
                    asks,
                    event_timestamp,
                } => format!(
                    "TradeData - symbol: {}, bids: {:?}, asks: {:?}, event_timestamp: {}",
                    symbol, bids, asks, event_timestamp
                ),
            },
            OwnedLogMsg::Error {
                error_code,
                error_message,
            } => format!("Error Code: {}, Message {}", error_code, error_message),
        }
    }
}
