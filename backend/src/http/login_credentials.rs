use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web::{Data, Json},
    Error, HttpResponse,
};
use log::debug;
use webapp::protocol::{request::LoginCredentials, response::Login};

pub async fn login_credentials(
    payload: Json<LoginCredentials>,
) -> Result<HttpResponse, Error> {
    let r = payload.into_inner();

    debug!("User {} is trying to login", r.username);
    if r.username.is_empty() || r.password.is_empty() || r.username != r.password {
        return Err(ErrorUnauthorized("wrong username or password"));
    }
    Ok(HttpResponse::Ok().json("Login successful"))
}