use serde::Deserialize;

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