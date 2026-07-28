use chrono::{DateTime, Local};
use tokio::runtime::Runtime;
use crate::weather::{self, Weather};


const WEATHER_SIZE: usize = 3;

#[derive(Debug)]
pub struct HourForecast {
    pub time: String,
    pub temperature: f64,
    pub precipitation: i32,
}

pub fn build_display_left(now: &DateTime<Local>) -> String {
    format!(
        "\n{}\n\n\n{}",
        now.format("%H:%M"),
        now.format("%d\n%b").to_string().to_uppercase(),
    )
}

pub fn build_display_right(now: &DateTime<Local>) -> String {
    let output: String = get_weather(now);
    format!("{}", output).to_string()
}

fn get_weather(now: &DateTime<Local>) -> String {
    let rt = Runtime::new().unwrap();
    // Update this hourly, move to a new function
    let api_data = rt.block_on(weather::get_weather(now)).unwrap();

    let forecast = current_weather(now, &api_data, WEATHER_SIZE);
    
    // Combine it into a single multiline string and return it
    forecast
        .iter()
        .map(|hour| {
            format!(
                "{}-----{} C\n{}\n\n",
                format_time(hour.time.clone()),
                hour.temperature.round(),
                format_precipitation(hour.precipitation)
            )
        })
        .collect::<Vec<String>>()
        .join("")
}


fn current_weather(now: &DateTime<Local>, weather: &Weather, count: usize) -> Vec<HourForecast> {
    // Format: "2026-07-27T14:00"
    let now = now.format("%Y-%m-%dT%H:00").to_string();
    let Some(start) = weather.hourly.time.iter().position(|t| t == &now) else {
        return Vec::new();
    };

    let end = (start + count).min(weather.hourly.time.len());

    (start..end)
        .map(|i| HourForecast {
            time: weather.hourly.time[i][11..13].to_string(), // Get the hour only
            temperature: weather.hourly.temperature_2m[i],
            precipitation: weather.hourly.precipitation_probability[i],
        })
        .collect()
}

fn format_time(mut time: String) -> String {
    let num: i32 = time.parse().unwrap();

    if num < 12 {
        time.push_str(" AM");
    } else {
        time.push_str(" PM");
    }
    time
}

fn format_precipitation(percipitation: i32) -> String {
    let mut percip_str: String = percipitation.to_string();

    if percipitation < 10 {
        percip_str.push_str("  %");
    } else {
        percip_str.push_str(" %");
    }
    percip_str
}