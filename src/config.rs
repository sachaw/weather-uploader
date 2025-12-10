use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub wu_station_id: String,
    pub wu_password: String,
    pub pws_station_id: String,
    pub pws_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            wu_station_id: env::var("WU_STATION_ID").context("WU_STATION_ID not set")?,
            wu_password: env::var("WU_PASSWORD").context("WU_PASSWORD not set")?,
            pws_station_id: env::var("PWS_STATION_ID").context("PWS_STATION_ID not set")?,
            pws_password: env::var("PWS_PASSWORD").context("PWS_PASSWORD not set")?,
        })
    }
}
