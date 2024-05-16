use actix_web::{middleware, web::{self, post, resource}, App, HttpServer};
use webapp::{config::Config, API_URL_LOGIN_CREDENTIALS, API_URL_LOGIN_SESSION, API_URL_LOGOUT};
use anyhow::{format_err, Result};
use url::Url;
use actix::{prelude::*, SystemRunner};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    slice::from_ref,
    thread,
};

/// The server instance
pub struct Server {
    config: Config,
    runner: SystemRunner,
    url: Url,
}


impl Server {
    pub fn from_config(config: &Config) -> Result<Self> {
        // Actor system
        let runner = actix::System::new();

        let server = HttpServer::new(move || {
            App::new()
               .wrap(middleware::Logger::default())
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
        unimplemented!()
    }

     /// Convert an `Url` to a vector of `SocketAddr`
     pub fn url_to_socket_addrs(url: &Url) -> Result<Vec<SocketAddr>> {
        let host = url
            .host()
            .ok_or_else(|| format_err!("No host name in the URL"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| format_err!("No port number in the URL"))?;
        let addrs;
        let addr;
        Ok(match host {
            url::Host::Domain(domain) => {
                addrs = (domain, port).to_socket_addrs()?;
                addrs.as_slice().to_owned()
            }
            url::Host::Ipv4(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
            url::Host::Ipv6(ip) => {
                addr = (ip, port).into();
                from_ref(&addr).to_owned()
            }
        })
    }
}