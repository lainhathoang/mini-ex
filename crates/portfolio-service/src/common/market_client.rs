use std::str::FromStr;

use database::sea_orm::prelude::Decimal;
use serde::Deserialize;

/// Why a price lookup against the market-service failed.
///
/// Maps directly onto the order [`RejectReason`](database::enums::RejectReason)
/// values the caller must persist when an order cannot be priced.
pub enum PriceError {
    /// The market-service could not be reached or returned an unexpected status.
    Unavailable,
    /// The symbol is unknown to the market-service (HTTP 404) or unparseable.
    InvalidSymbol,
}

/// Shape of the market-service `GET /prices/{symbol}` response we depend on.
#[derive(Deserialize)]
struct PriceResponse {
    price: String,
}

/// Fetches the latest USD price for `symbol` from the market-service.
///
/// Reuses the market-service HTTP API (never CoinGecko directly). Symbols are
/// stored lowercase by the market-service, so the lookup is normalized here.
pub async fn fetch_price(
    client: &reqwest::Client,
    base_url: &str,
    symbol: &str,
) -> Result<Decimal, PriceError> {
    let url = format!(
        "{}/prices/{}",
        base_url.trim_end_matches('/'),
        symbol.to_lowercase()
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| PriceError::Unavailable)?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(PriceError::InvalidSymbol);
    }

    if !response.status().is_success() {
        return Err(PriceError::Unavailable);
    }

    let body = response
        .json::<PriceResponse>()
        .await
        .map_err(|_| PriceError::Unavailable)?;

    Decimal::from_str(&body.price).map_err(|_| PriceError::InvalidSymbol)
}
