use crate::{raw_func_logger::Formattable, types::ExampleOB};
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::thread;

pub struct EnumLogger<EnumType: Formattable> {
    sender: Sender<EnumType>,
}

impl<EnumType: Formattable + Send + 'static> EnumLogger<EnumType> {
    pub fn new() -> Self {
        let (mut tx, mut rx) = create::<EnumType>();

        let guard = thread::spawn(move || {
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(*core_ids.last().unwrap());

            match (rx.recv()) {
                Ok(msg) => println!("ob update received"),
                Err(e) => panic!("receive error "),
            }
        });

        EnumLogger { sender: tx }
    }

    pub async fn log(&mut self, msg: EnumType) {
        self.sender.send(msg);
    }
}
