#![allow(warnings)]
use crate::example_types::{EventTypes, LogMsg, OwnedLogMsg};
use anyhow::Result;
use chrono::Utc;
use lockfree::channel::spsc::create;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub async fn async_logger<G>(log_message: Arc<G>) -> Result<()>
where
    G: Formattable + Clone + Send + Sync + 'static,
{
    let (mut sx, mut rx) = create::<RawFunc>();
    thread::spawn(move || {
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

// RawFunc: lock-free fn pointer
pub struct RawFunc {
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

pub struct Logger<'a> {
    formats: HashMap<LogMsg<'a>, String>,
}
impl<'a> Logger<'a> {
    fn new() -> Self {
        Self {
            formats: HashMap::new(),
        }
    }
    fn log(&self, message: &LogMsg) {
        let formatted_msg = message.format();

        //NOTE: Add actual logging/send/publish method below
        // println!("{:?}", formatted_msg);
    }
    fn log_formatting(&self, message: OwnedLogMsg) {
        let formatted_msg = message.formatting();
    }
}
pub trait Formattable {
    fn formatting(&self) -> String;
}

pub trait ToLogMsg {
    fn to_log_msg(self) -> OwnedLogMsg;
}

pub struct OwnedDataLogger {
    pub formats: HashMap<OwnedLogMsg, String>,
}

impl OwnedDataLogger {
    fn new() -> Self {
        Self {
            formats: HashMap::new(),
        }
    }
    fn log(&self, message: LogMsg) {
        let formatted_msg = message.format();
        //NOTE: Add actual logging/send/publish method below
        // println!("{:?}", formatted_msg);
    }
}

pub struct LoggerWithContext<G>
where
    G: Clone + Send + Sync + Formattable,
{
    logger: OwnedDataLogger,
    log_message: Arc<G>,
}
impl<G> LoggerWithContext<G>
where
    G: Formattable + Clone + Send + Sync,
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
        let formatted = self.log_message.formatting();
        //NOTE: Add actual logging/send/publish method below
        // println!("{:?}", formatted);
    }
}

// Old Function, retaining for benchmark
pub async fn async_logging_thread() -> Result<()> {
    let (mut sx, mut rx) = create::<RawFunc>();
    thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());
        while let Ok(raw_func) = rx.recv() {
            raw_func.invoke();
        }
    });

    /* Implementation Example */
    let logger = Logger::new();
    let ts = Utc::now().timestamp_millis();

    //NOTE commented example for LogMsg::Warning

    // let log_message = LogMsg::Warning {
    //     warning_message: "testing_message",
    // };
    // let ts_str: &str = ts;

    let log_message = LogMsg::Event(EventTypes::MarketTradesUpdate {
        symbol: "BTCUSDT",
        side: "buy",
        qty: "1",
        fill_price: "46030.50",
        timestamp: ts,
    });
    let raw_func = RawFunc::new(move || logger.log(&log_message));
    sx.send(raw_func);

    Ok(())
}
