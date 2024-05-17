mod test;

use crate::{
    database::DatabaseExecutor,
    http::{login_credentials, login_session, logout},
};
use actix::{prelude::*, SystemRunner};
use actix_web::{
    http::header::{CONTENT_TYPE, LOCATION},
    middleware,
    web::{self, get, post, resource},
    App, HttpResponse, HttpServer, HttpRequest,
};
use anyhow::{format_err, Result};
use diesel::{r2d2::ConnectionManager, IntoSql, PgConnection};
use dotenv::dotenv;
use log::{debug, error, info, warn};
use r2d2::Pool;
use std::{env, intrinsics::mir::Move};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    slice::from_ref,
    thread,
};
use url::{Host, Url};
use webapp::{config::Config, API_URL_LOGIN_CREDENTIALS, API_URL_LOGIN_SESSION, API_URL_LOGOUT};

/// The server instance
pub struct Server {
    config: Config,
    runner: SystemRunner,
    url: Url,
}

impl Server {
    /// Create a new server instance
    pub fn from_config(config: &Config) -> Result<Self> {
        dotenv().ok();

        // Actor system
        let runner = actix::System::new();

        // database executor actors
        let database_url = env::var("DATABASE_URL").map_err(|e| {
            error!("DATABASE_URL not set in .env: {:?}", e);
            e
        })?;

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)?;
        let db_addr = SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()));

        let server = HttpServer::new(move || {
            App::new()
                .app_data(db_addr.clone())
                .wrap(middleware::Logger::default())
                .service(resource(API_URL_LOGIN_CREDENTIALS).route(post().to(login_credentials)))
                .service(resource(API_URL_LOGIN_SESSION).route(post().to(login_session)))
                .service(resource(API_URL_LOGOUT).route(post().to(logout)))
        });

        // server url from configuration
        let url = Url::parse(&config.server.url)?;

        // Bind adress
        let addrs = Self::url_to_socket_addrs(&url)?;

        if url.scheme() == "https" {
            todo!("Handle HTTPS configuration");
        } else {
            server.bind(addrs.as_slice())?.run();
        }

        Ok(Server {
            config: config.to_owned(),
            runner,
            url,
        })
    }

    pub fn start(self) -> Result<()> {
        // redirecting server
        self.start_redirects();

        // main server
        self.runner.run()?;
        Ok(())
    }

    fn start_redirects(&self) {
        // Check if we need to create a redirecting server
        if !self.config.server.redirect_from.is_empty() {
            // Prepare needed variables
            let server_url = self.url.clone();
            let urls = self.config.server.redirect_from.to_owned();
            let config_clone = self.config.clone();

            // Create a separate thread for redirecting
            thread::spawn(move || {
                let system = actix::System::new();
                let url = server_url.clone();

                // Create redirecting server
                let mut server = HttpServer::new(move || {
                    let location = url.clone();
                    App::new().service(resource("/").route(get().to(move || {
                        HttpResponse::PermanentRedirect()
                            .header(LOCATION, location.as_str())
                            .finish()
                    })))
                });

                // Bind the URLs if possible
                for url in &urls {
                    if let Ok(valid_url) = Url::parse(url) {
                        info!(
                            "Starting server to redirect from {} to {}",
                            valid_url, server_url
                        );
                        let addrs = Self::url_to_socket_addrs(&valid_url).unwrap();
                        if valid_url.scheme() == "https" {
                            todo!()
                        } else {
                            server = server.bind(addrs.as_slice()).unwrap();
                        }
                    } else {
                        warn!("Skipping invalid url: {}", url);
                    }
                }

                // Start the server and the system
                server.run();
                system.run().unwrap();
            });
        }
    }


    /// Convert an `Url` to a vector of `SocketAddr`
    pub fn url_to_socket_addrs(url: &Url) -> Result<Vec<SocketAddr>> {
        let host = url
            .host()
            .ok_or_else(|| format_err!("No host name in the URL"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| format_err!("No port number in the URL"))?;

        match host {
            Host::Domain(domain) => {
                let addrs: Vec<SocketAddr> = (domain, port).to_socket_addrs()?.collect();
                Ok(addrs)
            }
            Host::Ipv4(ip) => {
                let addr = SocketAddr::from((ip, port));
                Ok(vec![addr])
            }
            Host::Ipv6(ip) => {
                let addr = SocketAddr::from((ip, port));
                Ok(vec![addr])
            }
        }
    }
}