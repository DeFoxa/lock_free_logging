use crate::example_types::ExampleOB;
use lockfree::channel::spsc::{create, Receiver, Sender};
use std::thread;

pub async fn enum_logger(event: ExampleOB) {
    let (mut tx, mut rx) = create::<ExampleOB>();

    let guard = thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());

        // match (rx.recv()) {
        //     Ok(msg) => println!("ob update received"),
        //     Err(e) => panic!("receive error "),
        // }
    });
    tx.send(event);
}
