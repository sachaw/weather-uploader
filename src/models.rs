use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Telegraf metric format
#[derive(Debug, Deserialize)]
pub struct TelegrafMetric {
    pub name: String,
    pub timestamp: i64,
    pub tags: Option<HashMap<String, String>>,
    pub fields: HashMap<String, serde_json::Value>,
}

// Response sent back to Telegraf
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Batch {
    pub metrics: Vec<TelegrafMetric>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MetricsInput {
    Batch(Batch),
    Single(TelegrafMetric),
}

// Weather data extracted from Telegraf metrics
#[derive(Debug, Default)]
pub struct WeatherData {
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
    pub barometric_pressure: Option<f64>,
    pub wind_direction: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_gust: Option<f64>,
    pub rainfall_hourly: Option<f64>,
    pub rainfall_daily: Option<f64>,
    pub pm2_5: Option<f64>,
    pub pm10: Option<f64>,
    pub light_intensity: Option<f64>, // Lux - stored but not uploaded (not solar radiation)
    pub co2: Option<f64>,
}

impl WeatherData {
    // Extract weather fields from Telegraf field map
    pub fn from_fields(fields: &HashMap<String, f64>) -> Self {
        WeatherData {
            temperature: fields.get("temperature").copied(),
            humidity: fields.get("humidity").copied(),
            barometric_pressure: fields.get("barometric_pressure").copied(),
            wind_direction: fields.get("wind_direction").copied(),
            wind_speed: fields.get("wind_speed").copied(),
            wind_gust: fields.get("wind_gust").copied(),
            rainfall_hourly: fields.get("rainfall_hourly").copied(),
            rainfall_daily: fields.get("rainfall_daily").copied(),
            pm2_5: fields.get("pm2_5").copied(),
            pm10: fields.get("pm10").copied(),
            light_intensity: fields.get("light_intensity").copied(),
            co2: fields.get("co2").copied(),
        }
    }
}
