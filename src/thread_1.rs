use std::{thread, time};
use std::time::Duration;
use bus::BusReader;
use crate::{AppSignal, check_app_exit};

pub fn start_thread_1(signal_bus_reader: &mut BusReader<AppSignal>) {
    const THREAD_LOOP_INTERVAL: Duration = Duration::from_millis(10 * 1000);

    loop {
        log::info!("Hello from thread 1");

        if check_app_exit(signal_bus_reader, THREAD_LOOP_INTERVAL) {
            break;
        }
    }
}
