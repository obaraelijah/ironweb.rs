use serde::Deserialize;
use std::fs::read_to_string;
use std::error::Error;

#[derive(Deserialize, Config)]
pub struct Config {
    server,
    pub postgres: PostgresConfig,
}

#[derive(Deserialize, Config)]
pub struct PostgresConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content  = read_to_string(filename)?;
        let config: Self  = toml::from_str(&content)?;
        Ok(config)
    }
}