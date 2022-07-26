use std::{fmt, thread, time};
use std::sync::Arc;

use bus::{Bus, BusReader};
use crossbeam_channel::*;
use log;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppSignal {
    QUIT,
}

impl fmt::Display for AppSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn check_app_exit(rx: &mut BusReader<AppSignal>) -> bool {
    match rx.try_recv() {
        Ok(signal) => {
            return signal == AppSignal::QUIT;
        }

        Err(err) => {}
    }

    false
}

fn start_thread_1(rx: &mut BusReader<AppSignal>) {
    loop {
        if check_app_exit(rx) {
            break;
        }

        log::info!("Hello from thread 1");
        thread::sleep(time::Duration::from_millis(10 * 1000));
    }
}

fn start_thread_2(rx: &mut BusReader<AppSignal>) {
    loop {
        if check_app_exit(rx) {
            break;
        }

        log::info!("Hello from thread 2");
        thread::sleep(time::Duration::from_millis(150));
    }
}

const BUS_SIZE: usize = 10;

fn main() {
    env_logger::init();
    log::info!("Starting app");
    let mut bus = Bus::new(BUS_SIZE);
    let threads = [
        {
            let mut rx = bus.add_rx();
            thread::spawn(move || start_thread_1(&mut rx))
        },
        {
            let mut rx = bus.add_rx();
            thread::spawn(move || start_thread_2(&mut rx))
        }
    ];

    thread::sleep(time::Duration::from_secs(10));
    log::info!("Quitting app...");
    bus.broadcast(AppSignal::QUIT);

    for curr_thread in threads {
        log::info!("Waiting for {:?} to exit", curr_thread.thread().id());
        curr_thread.join().unwrap();
    }

    log::info!("Exited")
}
