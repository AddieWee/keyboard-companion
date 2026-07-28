use chrono::{Local, Timelike};
use std::{thread, time::Duration};

use crate::hid::DisplayDevice;

mod hid;
mod weather;
mod config;
mod construct_screen;

const REFRESH_INTERVAL: Duration = Duration::from_secs(1);



fn main() {
    let display = DisplayDevice::new()
        .expect("Could not connect to display");
    
    let now = Local::now();
    let mut last_minute = -1_i64;
    let current_hour = now.hour() as i64;
    let mut last_hour = -1_i64;
    let mut initial_run:bool = true;

    loop {
        let current_minute = now.timestamp() / 60;

        if current_minute != last_minute {
            last_minute = current_minute;

            let screen_left = construct_screen::build_display_left(&now);

            if let Err(e) = display.send_left(screen_left) {
                eprintln!("Failed to send display: {e}");
            }

            thread::sleep(Duration::from_millis(100));
        }

        // Run hourly or when app is first started
        if initial_run || current_hour!=last_hour {
            initial_run = false;
            last_hour = current_hour;
            let screen_right = construct_screen::build_display_right(&now);
            if let Err(e) = display.send_right(screen_right) {
                eprintln!("Failed to send display: {e}");
            }
        }

        thread::sleep(REFRESH_INTERVAL);
    }
}