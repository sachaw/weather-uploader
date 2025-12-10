use crate::config::Config;
use crate::conversions;
use crate::models::WeatherData;
use anyhow::{Context, Result};
use std::time::Duration;
use tracing::info;

const SOFTWARE_TYPE: &str = concat!(
    "Seed Studio SenseCAP S1000 Adapter v",
    env!("CARGO_PKG_VERSION")
);

const WU_URL: &str = "https://rtupdate.wunderground.com/weatherstation/updateweatherstation.php";
const PWS_URL: &str = "https://pwsupdate.pwsweather.com/api/v1/submitwx";

// Upload weather data to Weather Underground
pub async fn upload_to_weather_underground(config: &Config, data: &WeatherData) -> Result<()> {
    let mut params = vec![
        ("ID", config.wu_station_id.clone()),
        ("PASSWORD", config.wu_password.clone()),
        ("action", "updateraw".to_string()),
        ("dateutc", "now".to_string()),
        ("softwaretype", SOFTWARE_TYPE.to_string()),
    ];

    add_weather_params(&mut params, data);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let response = client
        .get(WU_URL)
        .query(&params)
        .send()
        .await
        .context("Failed to send request to Weather Underground")?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() && body.contains("success") {
        info!("✓ Weather Underground upload successful");
        Ok(())
    } else {
        anyhow::bail!("Weather Underground upload failed: {} - {}", status, body)
    }
}

// Upload weather data to PWSWeather
pub async fn upload_to_pwsweather(config: &Config, data: &WeatherData) -> Result<()> {
    let mut params = vec![
        ("ID", config.pws_station_id.clone()),
        ("PASSWORD", config.pws_password.clone()),
        ("action", "updateraw".to_string()),
        ("dateutc", "now".to_string()),
        ("softwaretype", SOFTWARE_TYPE.to_string()),
    ];

    add_weather_params(&mut params, data);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let response = client
        .get(PWS_URL)
        .query(&params)
        .send()
        .await
        .context("Failed to send request to PWSWeather")?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        info!("✓ PWSWeather upload successful");
        Ok(())
    } else {
        anyhow::bail!("PWSWeather upload failed: {} - {}", status, body)
    }
}

// Add weather parameters to request (common for both services)
fn add_weather_params(params: &mut Vec<(&str, String)>, data: &WeatherData) {
    // Temperature
    if let Some(temp) = data.temperature {
        params.push((
            "tempf",
            format!("{:.2}", conversions::celsius_to_fahrenheit(temp)),
        ));
    }

    // Humidity
    if let Some(humidity) = data.humidity {
        params.push(("humidity", format!("{:.0}", humidity)));
    }

    // Dewpoint (calculated)
    if let (Some(temp), Some(humidity)) = (data.temperature, data.humidity) {
        let dewpoint = conversions::calculate_dewpoint_f(temp, humidity);
        params.push(("dewptf", format!("{:.2}", dewpoint)));
    }

    // Barometric pressure
    if let Some(pressure) = data.barometric_pressure {
        params.push((
            "baromin",
            format!("{:.3}", conversions::hpa_to_inhg(pressure)),
        ));
    }

    // Wind direction
    if let Some(wind_dir) = data.wind_direction {
        params.push(("winddir", format!("{:.0}", wind_dir)));
    }

    // Wind speed
    if let Some(wind_speed) = data.wind_speed {
        params.push((
            "windspeedmph",
            format!("{:.2}", conversions::ms_to_mph(wind_speed)),
        ));
    }

    // Wind gust
    if let Some(wind_gust) = data.wind_gust {
        params.push((
            "windgustmph",
            format!("{:.2}", conversions::ms_to_mph(wind_gust)),
        ));
    }

    // Rainfall - hourly
    if let Some(rain_hourly) = data.rainfall_hourly {
        params.push((
            "rainin",
            format!("{:.3}", conversions::mm_to_inches(rain_hourly)),
        ));
    }

    // Rainfall - daily
    if let Some(rain_daily) = data.rainfall_daily {
        params.push((
            "dailyrainin",
            format!("{:.3}", conversions::mm_to_inches(rain_daily)),
        ));
    }

    // Note: S1000 has light_intensity (Lux) but not solar_radiation (W/m²)
    // These are different measurements and cannot be directly converted

    // Air quality - PM2.5
    if let Some(pm25) = data.pm2_5 {
        params.push(("AqPM2.5", format!("{:.1}", pm25)));
        params.push(("AqPM2.5_avg_24h", format!("{:.1}", pm25)));
    }

    // Air quality - PM10
    if let Some(pm10) = data.pm10 {
        params.push(("AqPM10", format!("{:.1}", pm10)));
        params.push(("AqPM10_avg_24h", format!("{:.1}", pm10)));
    }
}
