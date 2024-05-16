use std::env::set_var;

use anyhow::{format_err, Result};
use clap::{App, Arg};
use log::info;
use webapp::config::Config;
use backend::Server;

fn main() -> Result<()> {
    // Define CLI parameters using the App API
    let matches = App::new("webapp.rs")
        .bin_name("backend")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Elijah Samson <elijahobara357@gmail.com>")
        .about("The web server backend of ironweb application")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .long_help("Sets the config file to use")
                .takes_value(true)
                .default_value("../Config.toml"),
        )
        .get_matches();

    // Retrieve the config file path
    let config_filename = matches
        .value_of("config")
        .ok_or_else(|| format_err!("No 'config' provided"))?;

    // let config_path = PathBuf::from(config_filename);
    //     if !config_path.exists() {
    //         return Err(format_err!("Configuration file not found: {}", config_filename));
    //     }

    // Parse the configuration
    let config = Config::from_file(config_filename)?;

    // Set the logging verbosity
    set_var(
        "RUST_LOG",
        format!("webapp={},backend={}", config.log.webapp, config.log.webapp),
    );
    // Initialize the logger
    env_logger::init();

    // Create and start the server
    info!(
        "Starting server from config path {} for url {}",
        config_filename, config.server.url
    );
    let server = Server::from_config(&config)?;

    server.start()?;

    Ok(())
}