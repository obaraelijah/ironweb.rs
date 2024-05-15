#[cfg(feature = "backend")]
extern crate diesel;

pub mod config;
pub mod protocol;

#[cfg(feature = "backend")]
pub mod schema;

/// Global config filename
pub const CONFIG_FILENAME: &str = "Config.toml";

macro_rules! apis {
    ($($name:ident => $content:expr,)*) => (
        $(#[allow(missing_docs)] pub const $name: &str = $content;)*
    )
}

apis! {
    API_URL_LOGIN_CREDENTIALS => "login/credentials",
    API_URL_LOGIN_SESSION => "login/session",
    API_URL_LOGOUT => "logout",
}
