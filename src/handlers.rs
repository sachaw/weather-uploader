use crate::config::Config;
use crate::conversions;
use crate::models::{TelegrafMetric, UploadResponse, WeatherData};
use crate::upload::{upload_to_pwsweather, upload_to_weather_underground};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub latest_data: Arc<RwLock<HashMap<String, f64>>>,
}

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

// Handle metrics from Telegraf
pub async fn handle_metrics(
    State(state): State<AppState>,
    Json(metric): Json<TelegrafMetric>,
) -> impl IntoResponse {
    info!("Received metric from Telegraf: {}", metric.name);

    if metric.name != "weather" {
        warn!("Received non-weather metric: {}", metric.name);
        return (
            StatusCode::BAD_REQUEST,
            Json(UploadResponse {
                success: false,
                message: "Expected 'weather' metric".to_string(),
            }),
        );
    }

    // Extract fields
    let mut latest_fields = HashMap::new();
    for (field_name, value) in metric.fields {
        if let Some(num) = value.as_f64() {
            latest_fields.insert(field_name, num);
        }
    }

    if latest_fields.is_empty() {
        warn!("No fields found in weather metric");
        return (
            StatusCode::BAD_REQUEST,
            Json(UploadResponse {
                success: false,
                message: "No weather data found".to_string(),
            }),
        );
    }

    // Store latest data
    {
        let mut data = state.latest_data.write().await;
        *data = latest_fields.clone();
    }

    // Convert to WeatherData and upload
    let weather = WeatherData::from_fields(&latest_fields);

    log_weather_data(&weather);

    // Upload to both services
    let mut upload_errors = Vec::new();

    if let Err(e) = upload_to_weather_underground(&state.config, &weather).await {
        error!("Weather Underground upload failed: {:#}", e);
        upload_errors.push(format!("WU: {}", e));
    }

    if let Err(e) = upload_to_pwsweather(&state.config, &weather).await {
        error!("PWSWeather upload failed: {:#}", e);
        upload_errors.push(format!("PWS: {}", e));
    }

    if upload_errors.is_empty() {
        (
            StatusCode::OK,
            Json(UploadResponse {
                success: true,
                message: "Uploaded successfully to both services".to_string(),
            }),
        )
    } else {
        (
            StatusCode::PARTIAL_CONTENT,
            Json(UploadResponse {
                success: false,
                message: format!("Some uploads failed: {}", upload_errors.join(", ")),
            }),
        )
    }
}

// Log received weather data
fn log_weather_data(weather: &WeatherData) {
    info!("Weather data received:");

    if let Some(temp) = weather.temperature {
        info!(
            "  Temperature: {:.1}°C ({:.1}°F)",
            temp,
            conversions::celsius_to_fahrenheit(temp)
        );
    }

    if let Some(humidity) = weather.humidity {
        info!("  Humidity: {:.0}%", humidity);
    }

    if let Some(pressure) = weather.barometric_pressure {
        info!(
            "  Pressure: {:.1} hPa ({:.2} inHg)",
            pressure,
            conversions::hpa_to_inhg(pressure)
        );
    }

    if let Some(wind_speed) = weather.wind_speed {
        info!(
            "  Wind: {:.1} m/s ({:.1} mph)",
            wind_speed,
            conversions::ms_to_mph(wind_speed)
        );
    }
}
