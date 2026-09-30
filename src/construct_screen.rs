use chrono::{NaiveDateTime, DateTime, Local};
use tokio::runtime::Runtime;
use crate::weather::{self, AirQuality, Weather};


const WEATHER_SIZE: usize = 3;

#[derive(Debug)]
pub struct HourForecast {
    pub time: String,
    pub temperature: f64,
    pub precipitation: i32,
}

pub fn build_display_left(now: &DateTime<Local>, aqi: Option<i32>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "\n{}\n\n\n{}\n\n{}",
        now.format("%H:%M"),
        now.format("%d\n%b").to_string().to_uppercase(),
        format_aqi(aqi),
    ))
}

pub fn build_display_right(now: &DateTime<Local>) -> Result<String, Box<dyn std::error::Error>>  {
    get_weather(now)
}

fn get_weather(now: &DateTime<Local>) -> Result<String, Box<dyn std::error::Error>>  {
    let rt = Runtime::new().unwrap();
    let api_data = rt.block_on(weather::get_weather(now))?;
    let forecast = current_weather(now, &api_data, WEATHER_SIZE);

    Ok(
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
            .collect::<Vec<_>>()
            .join("")
    )
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
            time: weather.hourly.time[i].clone(),
            temperature: weather.hourly.temperature_2m[i],
            precipitation: weather.hourly.precipitation_probability[i],
        })
        .collect()
}

pub fn get_current_aqi(now: &DateTime<Local>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    let air_quality = rt.block_on(weather::get_air_quality(now))?;
    Ok(aqi_at(&air_quality, &now.format("%Y-%m-%dT%H:00").to_string()))
}

fn aqi_at(air_quality: &AirQuality, time: &str) -> Option<i32> {
    let hourly = &air_quality.hourly;
    let index = hourly.time.iter().position(|t| t == time)?;
    hourly.us_aqi.get(index).copied().flatten()
}

fn format_aqi(aqi: Option<i32>) -> String {
    aqi.map_or_else(|| "-".to_string(), |v| format!("AQI\n{v}"))
}

fn format_time(datetime: String) -> String {
    let dt = NaiveDateTime::parse_from_str(datetime.as_str(), "%Y-%m-%dT%H:%M").unwrap();
    dt.format("%I %p").to_string()
}

fn format_precipitation(percipitation: i32) -> String {
    let mut percip_str: String = percipitation.to_string();

    if percipitation < 10 {
        percip_str.push_str("  %");
    } else if percipitation < 100 {
        percip_str.push_str(" %");
    } else {
        percip_str.push_str("%");
    }
    percip_str
}
