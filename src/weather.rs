use reqwest::Client;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use crate::config;
use chrono::{DateTime, Local, Duration};

type BoxError = Box<dyn std::error::Error>;

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const AIR_QUALITY_URL: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";
const FORECAST_HOURS: i64 = 3;

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

#[derive(Debug, Deserialize)]
pub struct AirQuality {
    pub hourly: AirQualityHourly,
}

#[derive(Debug, Deserialize)]
pub struct AirQualityHourly {
    pub time: Vec<String>,
    // Open-Meteo returns null for hours without data
    pub us_aqi: Vec<Option<i32>>,
}

struct Location {
    latitude: String,
    longitude: String,
    timezone: String,
}

pub async fn get_weather(now: &DateTime<Local>) -> Result<Weather, BoxError> {
    fetch_hourly(FORECAST_URL, "temperature_2m,precipitation_probability", now).await
}

pub async fn get_air_quality(now: &DateTime<Local>) -> Result<AirQuality, BoxError> {
    fetch_hourly(AIR_QUALITY_URL, "us_aqi", now).await
}

async fn fetch_hourly<T: DeserializeOwned>(
    url: &str,
    hourly_fields: &str,
    now: &DateTime<Local>,
) -> Result<T, BoxError> {
    let location = get_location()?;
    let (start_date, end_date) = get_dates(now, FORECAST_HOURS);

    let response = Client::new()
        .get(url)
        .query(&[
            ("latitude", location.latitude.as_str()),
            ("longitude", location.longitude.as_str()),
            ("hourly", hourly_fields),
            ("timezone", location.timezone.as_str()),
            ("start_date", start_date.as_str()),
            ("end_date", end_date.as_str()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<T>()
        .await?;

    Ok(response)
}

fn get_location() -> Result<Location, BoxError> {
    let latitude = config::get("latitude")
        .and_then(|v| v.as_float())
        .ok_or("Missing latitude")?
        .to_string();

    let longitude = config::get("longitude")
        .and_then(|v| v.as_float())
        .ok_or("Missing longitude")?
        .to_string();

    let timezone = config::get("timezone")
        .and_then(|v| v.as_str())
        .ok_or("Missing timezone")?
        .to_string();

    Ok(Location { latitude, longitude, timezone })
}

fn get_dates(now: &DateTime<Local>, hours: i64) -> (String, String) {
    let start_date = now.format("%Y-%m-%d").to_string();

    let end_date = (*now + Duration::hours(hours))
        .format("%Y-%m-%d")
        .to_string();

    (start_date, end_date)
}

