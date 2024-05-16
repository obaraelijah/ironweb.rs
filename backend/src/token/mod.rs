use actix_web::{ HttpResponse, ResponseError};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token creation error")]
    Create,
    #[error("Token verification error")]
    Verify,
}

impl ResponseError for TokenError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TokenError::Create => HttpResponse::InternalServerError().into(),
            TokenError::Verify => HttpResponse::Unauthorized().into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
/// A web token
pub struct Token {
    sub: String,
    exp: i64,
    /// The issued at field
    iat: i64,
    /// The token id
    jti: String,
}

impl Token {
    pub fn create(username: &str) -> Result<String, TokenError> {
        unimplemented!()
    }

    pub fn verify(token: &str) -> Result<String, TokenError> {
        unimplemented!()
    }
 }