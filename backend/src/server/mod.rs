mod test;

use crate::database::DatabaseExecutor;
use crate::http::login_credentials::login_credentials;
use actix::{prelude::*, SystemRunner};
use actix_web::{
    middleware,
    web::{self, post, resource},
    App, HttpServer,
};
use anyhow::{format_err, Ok, Result};
use diesel::{r2d2::ConnectionManager, IntoSql, PgConnection};
use r2d2::Pool;
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
        // Actor system
        let runner = actix::System::new();

        // database executor actors
        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            config.postgres.username,
            config.postgres.password,
            config.postgres.host,
            config.postgres.database,
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)?;
        let db_addr = SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()));

        let server = HttpServer::new(move || {
            App::new()
                .app_data(db_addr.clone())
                .wrap(middleware::Logger::default())
                .service(resource(API_URL_LOGIN_CREDENTIALS).route(post().to(login_credentials)))
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
        self.runner.run()?;

        Ok(())
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
