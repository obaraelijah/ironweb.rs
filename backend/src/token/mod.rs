mod test;

use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

const SECRET: &[u8] = b"my_secret";

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
        const DEFAULT_TOKEN_VALIDITY: i64 = 3600;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TokenError::Create)?;

        let claim = Token {
            sub: username.to_owned(),
            exp: now.as_secs() as i64 + DEFAULT_TOKEN_VALIDITY,
            iat: now.as_secs() as i64,
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(SECRET),
        )
        .map_err(|_| TokenError::Create)
    }

    pub fn verify(token: &str) -> Result<String, TokenError> {
        let data = decode::<Token>(
            token,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .map_err(|_| TokenError::Verify)?;
        Self::create(&data.claims.sub)
    }
}
