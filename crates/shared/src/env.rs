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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn env_keys_map_to_expected_names() {
//         assert_eq!(Env::MarketDatabaseUrl.key().as_ref(), "MARKET_DATABASE_URL");
//         assert_eq!(
//             Env::PortfolioDatabaseUrl.key().as_ref(),
//             "PORTFOLIO_DATABASE_URL"
//         );
//     }
// }
