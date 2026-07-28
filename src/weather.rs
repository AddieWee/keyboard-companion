use reqwest::Client;
use serde::Deserialize;
use crate::config;
use chrono::{DateTime, Local, Duration};

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub hourly: Hourly,
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation_probability: Vec<i32>
}

pub async fn get_weather(now: &DateTime<Local>) -> Result<Weather, Box<dyn std::error::Error>> {
    let latitude = config::get("latitude")
        .and_then(|v| v.as_float())
        .expect("Missing latitude")
        .to_string();

    let longitude = config::get("longitude")
        .and_then(|v| v.as_float())
        .expect("Missing longitude")
        .to_string();

    let timezone = config::get("timezone")
    .and_then(|v| v.as_str())
    .expect("Missing timezone")
    .to_string();

    let (start_date, end_date) = get_dates(&now, 3);


    let client = Client::new();

    let weather = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", latitude),
            ("longitude", longitude),
            ("hourly", "temperature_2m,precipitation_probability".to_string()),
            ("timezone", timezone),
            ("start_date", start_date),
            ("end_date", end_date)
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<Weather>()
        .await?;

    Ok(weather)
}

fn get_dates(now: &DateTime<Local>, hours: i64) -> (String, String) {
    let start_date = now.format("%Y-%m-%d").to_string();

    let end_date = (*now + Duration::hours(hours))
        .format("%Y-%m-%d")
        .to_string();

    (start_date, end_date)
}

