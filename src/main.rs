use chrono::{Local, Timelike};
use std::{thread, time::Duration};

use crate::hid::DisplayDevice;

mod hid;
mod weather;
mod config;
mod construct_screen;

fn main() {
    let mut display: Option<DisplayDevice> = None;
    let mut last_minute = -1_i64;
    let mut last_hour_right = -1_i64;
    let mut aqi: Option<i32> = None;
    let mut last_hour_left = -1_i64;

    loop {
        let now = Local::now();
        let current_hour = now.hour() as i64;
        let current_minute = now.timestamp() / 60;

        if current_minute != last_minute {
            last_minute = current_minute;

           // If display isn't connected, try to reconnect
            if display.is_none() {
                match DisplayDevice::new() {
                    Ok(device) => {
                        println!("Connected!");
                        display = Some(device);
                        last_hour_right = -1_i64;
                        last_hour_left = -1_i64;
                    }
                    Err(err) => {
                        eprintln!("Could not connect to display: {}", err);
                        display = None;
                        continue;
                    }
                }
            }

            // AQI data is hourly, so avoid refetching every minute
            if current_hour != last_hour_left {
                match construct_screen::get_current_aqi(&now) {
                    Ok(value) => {
                        aqi = value;
                        last_hour_left = current_hour;
                    }
                    Err(err) => {
                        aqi = None;
                        eprintln!("Failed to fetch AQI: {err}");
                    }
                }
            }

            match construct_screen::build_display_left(&now, aqi) {
                Ok(screen_left) => {
                    match display.as_mut().unwrap().send_left(screen_left) {
                        Ok(()) => {}
                        Err(err) => {
                            display = None;
                            eprintln!("Failed to send data to left screen: {err}");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to build left screen: {}", err);
                }
            }
            
            thread::sleep(Duration::from_millis(100));

            // Run hourly || when app is first started || retry if failed
            if current_hour!=last_hour_right {

                match construct_screen::build_display_right(&now) {
                    Ok(screen_right) => {
                        match display.as_mut().unwrap().send_right(screen_right) {
                            Ok(()) => { 
                                last_hour_right = current_hour;
                            }
                            Err(err) => {
                                display = None;
                               eprintln!("Failed to send data to right screen: {err}");
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to build right screen: {}", err);
                    }
                }            
            }
        }

        let millis_until_next_minute = 60_000 - (now.timestamp_millis().rem_euclid(60_000) as u64);
        thread::sleep(Duration::from_millis(millis_until_next_minute));
    }
}
