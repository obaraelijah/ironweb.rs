use actix_web::{web, App, HttpServer};
use webapp::{config::Config, API_URL_LOGIN_CREDENTIALS, API_URL_LOGIN_SESSION, API_URL_LOGOUT};
use anyhow::Result;

pub struct Server {
    url: String,
}

impl Server {
    pub fn from_config(config: &Config) -> Result<Self> {
        HttpServer::new(move || {
            App::new()
                // Configure your Actix Web app routes here
                .route("/", web::get().to(|| async { "Hello, Actix Web!" }))
        })
        .bind("127.0.0.1:8080")?
        .run();
    }

    pub fn start(self) -> Result<()> {
        unimplemented!()
    }
}