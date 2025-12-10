use anyhow::{Context, Result};
use axum::{Router, routing::post};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use weather_uploader::{config::Config, handlers};

const LISTEN_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting weather uploader HTTP service");

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env().context("Failed to load configuration")?;

    info!("Configuration loaded:");
    info!("  WU Station: {}", config.wu_station_id);
    info!("  PWS Station: {}", config.pws_station_id);

    let state = handlers::AppState {
        config,
        latest_data: Arc::new(RwLock::new(HashMap::new())),
    };

    // Build router
    let app = Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
        .route("/metrics", post(handlers::handle_metrics))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDR)
        .await
        .context("Failed to bind to address")?;

    info!("HTTP server listening on {}", LISTEN_ADDR);
    info!("Ready to receive data from Telegraf!");
    info!("Telegraf should POST to: http://weather-uploader:8080/metrics");

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}
