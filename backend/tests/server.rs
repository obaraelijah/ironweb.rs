use anyhow::{format_err, Result};
use lazy_static::lazy_static;
use log::info;
use reqwest::blocking::Client;
use serde_json::from_slice;
use std::{sync::Mutex, thread, time::Duration};
use url::Url;
use webapp::{
    config::Config,
    protocol::{model::Session, request, response},
    API_URL_LOGIN_CREDENTIALS, API_URL_LOGIN_SESSION, API_URL_LOGOUT, CONFIG_FILENAME,
};

use backend::Server;

lazy_static! {
    static ref PORT: Mutex<u16> = Mutex::new(3000);
}

fn get_config() -> Result<Config> {
    Ok(Config::from_file(&format!("../{}", CONFIG_FILENAME))?)
}

fn get_next_port() -> u16 {
    let mut port = PORT.lock().unwrap();
    *port += 1;
    *port
}

pub fn create_test_server() -> Result<Url> {
    let mut config = get_config()?;

    // test configuration
    let mut url = Url::parse(&config.server.url)?;
    url.set_port(Some(get_next_port()))
        .map_err(|_| format_err!("Unable to set server port"))?;

    config.server.url = url.to_string();
    config.server.redirect_from = vec![];

    // start server
    thread::spawn(move || Server::from_config(&config.clone()).unwrap().start());

    // wait untill server is up
    loop {
        if let Ok(res) = Client::new().get(url.as_str()).send() {
            if res.status().is_success() {
                break;
            }
        }
        thread::sleep(Duration::from_secs(1))
    }

    // Server url
    Ok(url)
}

#[test]
fn succeed_to_create_server_with_common_redirects() -> Result<()> {
    // Given
    let mut config = get_config()?;
    let mut url = Url::parse(&config.server.url)?;
    url.set_port(Some(get_next_port()))
        .map_err(|_| format_err!("Unable to set server port"))?;
    config.server.url = url.to_string();

    let redirect_url = "http://127.0.0.1:30666".to_owned();
    config.server.redirect_from = vec![
        redirect_url.clone(),
        "https://localhost:30667".to_owned(),
        "invalid".to_owned(),
    ];

    // When
    let config_clone = config.clone();
    thread::spawn(move || {
        Server::from_config(&config_clone).unwrap().start().unwrap();
    });

    loop {
        match Client::new().get(url.as_str()).send() {
            Ok(res) => {
                if res.status().is_success() {
                    info!("Server started successfully.");
                    break;
                }
            }
            Err(e) => {
                info!("Waiting for server to start: {:?}", e);
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    let res = Client::new().get(&redirect_url).send()?;
    let final_url = res.url().to_string();

    // Then
    assert!(
        !final_url.contains(&redirect_url),
        "Redirect did not occur as expected."
    );
    assert_eq!(final_url, url.to_string(), "Redirected to incorrect URL.");

    Ok(())
}

#[test]
fn succeed_to_login_with_credentials() -> Result<()> {
    // Given
    let mut url = create_test_server()?;
    url.set_path(API_URL_LOGIN_CREDENTIALS);

    // When
    let request = request::LoginCredentials {
        username: "username".to_owned(),
        password: "password".to_owned(),
    };
    let mut res = Client::new().post(url.as_str()).json(&request).send()?;
    let mut body = vec![];
    res.copy_to(&mut body)?;
    let response::Login(session) = from_slice(&body)?;

    // Then
    assert!(res.status().is_success());
    assert_eq!(session.token.len(), 211);
    Ok(())
}

#[test]
fn fail_to_login_with_wrong_credentials() -> Result<()> {
    // Given
    let mut url = create_test_server()?;
    url.set_path(API_URL_LOGIN_CREDENTIALS);

    // When
    let request = request::LoginCredentials {
        username: "username".to_owned(),
        password: "password".to_owned(),
    };
    let res = Client::new().post(url.as_str()).json(&request).send()?;

    // Then
    assert_eq!(res.status().as_u16(), 401);
    Ok(())
}

#[test]
fn succeed_to_login_with_session() -> Result<()> {
    // Given
    let mut url = create_test_server()?;
    url.set_path(API_URL_LOGIN_CREDENTIALS);

    // When
    let request = &request::LoginCredentials {
        username: "username".to_owned(),
        password: "username".to_owned(),
    };
    let mut res = Client::new().post(url.as_str()).json(&request).send()?;
    let mut body = vec![];
    res.copy_to(&mut body)?;
    let response::Login(session) = from_slice(&body)?;

    url.set_path(API_URL_LOGIN_SESSION);
    res = Client::new()
        .post(url.as_str())
        .json(&request::LoginSession(session))
        .send()?;
    body.clear();
    res.copy_to(&mut body)?;
    let response::Login(new_session) = from_slice(&body)?;

    // Then
    assert!(res.status().is_success());
    assert_eq!(new_session.token.len(), 211);
    Ok(())
}

#[test]
fn fail_to_login_with_wrong_session() -> Result<()> {
    // Given
    let mut url = create_test_server()?;
    url.set_path(API_URL_LOGIN_SESSION);

    // When
    let res = Client::new()
        .post(url.as_str())
        .json(&request::LoginSession(Session::new("wrong")))
        .send()?;

    // Then
    assert_eq!(res.status().as_u16(), 401);
    Ok(())
}

#[test]
fn succeed_to_logout() -> Result<()> {
    // Given
    let mut url = create_test_server()?;
    url.set_path(API_URL_LOGOUT);

    // When
    let res = Client::new()
        .post(url.as_str())
        .json(&request::Logout(Session::new("wrong")))
        .send()?;

    // Then
    assert!(res.status().is_success());
    Ok(())
}
