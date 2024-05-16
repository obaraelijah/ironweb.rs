use crate::protocol::model::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// The session based login request
pub struct LoginSession(pub Session);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Logout(pub Session);
