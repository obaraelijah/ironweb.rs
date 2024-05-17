use anyhow::Result;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub postgres: PostgresConfig,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub url: String,

    // Server certificate
    pub cert: PathBuf,

    // Server key
    pub key: PathBuf,

    // redirecting URLs
    pub redirect_from: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct LogConfig {
    /// The logging level of actix-web
    pub actix_web: String,

    /// The logging level of the application
    pub webapp: String,
}
#[derive(Deserialize, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
}
