mod thread_2;
mod thread_1;

use std::{fmt, thread, time};
use std::sync::Arc;
use std::time::Duration;

use bus::{Bus, BusReader};
use crossbeam_channel::*;
use log;

use crate::thread_1::start_thread_1;
use crate::thread_2::start_thread_2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppSignal {
    QUIT,
}

impl fmt::Display for AppSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn check_app_exit(reader: &mut BusReader<AppSignal>, timeout: Duration) -> bool {
    match reader.recv_timeout(timeout) {
        Ok(signal) => {
            return signal == AppSignal::QUIT;
        }

        Err(err) => {}
    }

    false
}

const BUS_SIZE: usize = 10;

fn main() {
    env_logger::init();

    log::info!("Starting app");
    let mut main_signal_bus = Bus::new(BUS_SIZE);

    let threads = [
        {
            let mut rx = main_signal_bus.add_rx();
            thread::spawn(move || start_thread_1(&mut rx))
        },
        {
            let mut rx = main_signal_bus.add_rx();
            thread::spawn(move || start_thread_2(&mut rx))
        }
    ];

    thread::sleep(time::Duration::from_secs(10));
    log::info!("Quitting app...");
    main_signal_bus.broadcast(AppSignal::QUIT);

    for curr_thread in threads {
        log::info!("Waiting for {:?} to exit", curr_thread.thread().id());
        curr_thread.join().unwrap();
    }

    log::info!("Exited")
}
