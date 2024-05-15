/// Basic models
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Session {
    /// The actual session token
    pub token: String,
}
