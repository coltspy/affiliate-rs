use anyhow::{Context, Result};

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .context("PORT must be a valid number")?,
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            api_key: std::env::var("API_KEY")
                .context("API_KEY must be set")?,
        })
    }
}