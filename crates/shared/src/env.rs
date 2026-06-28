use std::borrow::Cow;

use strum::Display;

use crate::result::Rs;

#[derive(Debug, Display)]
pub enum Env {
    MarketDatabaseUrl,
    PortfolioDatabaseUrl,
    AccessTokenKey,
    CoingeckoApiKey,
    PortfolioServicePort,
    MarketServicePort,
    MarketServiceUrl,
}

// Docker tự bơm biến env vào process environment nên comment tạm lại
pub fn load() {
    // match dotenv::dotenv() {
    // Ok(path) => println!("Loaded .env file from: {}", path.display()),
    // Err(_) => println!("No .env file found, using system environment variables"),
    // }
}

pub fn read(env: Env) -> Rs<String> {
    std::env::var(env.key().as_ref()).map_err(Into::into)
}

impl Env {
    fn key(&self) -> Cow<'static, str> {
        match self {
            Self::MarketDatabaseUrl => "MARKET_DATABASE_URL".into(),
            Self::PortfolioDatabaseUrl => "PORTFOLIO_DATABASE_URL".into(),
            Self::AccessTokenKey => "ACCESS_TOKEN_KEY".into(),
            Self::CoingeckoApiKey => "COINGECKO_API_KEY".into(),
            Self::PortfolioServicePort => "PORTFOLIO_SERVICE_PORT".into(),
            Self::MarketServicePort => "MARKET_SERVICE_PORT".into(),
            Self::MarketServiceUrl => "MARKET_SERVICE_URL".into(),
        }
    }
}
