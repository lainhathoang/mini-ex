use std::borrow::Cow;

use strum::Display;

use crate::result::Rs;

#[derive(Debug, Display)]
pub enum Env {
    DatabaseUrl,
    AccessTokenKey,
    CoingeckoApiKey,
    HttpServerPort,
    MarketServicePort,
}

/// Loads environment variables from .env file if present
pub fn load() {
    match dotenv::dotenv() {
        Ok(path) => println!("Loaded .env file from: {}", path.display()),
        Err(_) => println!("No .env file found, using system environment variables"),
    }
}

/// Reads an environment variable, panicking with a clear message if missing
pub fn read(env: Env) -> Rs<String> {
    std::env::var(env.key().as_ref()).map_err(Into::into)
}

impl Env {
    fn key(&self) -> Cow<'static, str> {
        match self {
            Self::DatabaseUrl => "DATABASE_URL".into(),
            Self::AccessTokenKey => "ACCESS_TOKEN_KEY".into(),
            Self::CoingeckoApiKey => "COINGECKO_API_KEY".into(),
            Self::HttpServerPort => "HTTP_SERVER_PORST".into(),
            Self::MarketServicePort => "MARKET_SERVICE_PORT".into(),
        }
    }
}
