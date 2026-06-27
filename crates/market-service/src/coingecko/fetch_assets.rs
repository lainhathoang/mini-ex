use std::str::FromStr;

use database::{models::assets::AssetRecord, sea_orm::prelude::Decimal};
use serde::Deserialize;
use shared::result::{AppErr, Rs};

/// CoinGecko markets endpoint: top 100 coins by market cap, priced in USD.
const COINGECKO_MARKETS_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1";

/// Raw market entry as returned by the CoinGecko markets endpoint.
///
/// Only the fields persisted as [`AssetRecord`] are deserialized.
#[derive(Debug, Deserialize)]
struct CoinGeckoMarket {
    name: String,
    symbol: String,
    current_price: serde_json::Number,
}

/// Fetches the current top markets from CoinGecko and maps them to [`AssetRecord`]s.
pub(super) async fn fetch_assets(client: &reqwest::Client, api_key: &str) -> Rs<Vec<AssetRecord>> {
    let markets = client
        .get(COINGECKO_MARKETS_URL)
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<CoinGeckoMarket>>()
        .await?;

    markets
        .into_iter()
        .map(|market| {
            let price = Decimal::from_str(&market.current_price.to_string()).map_err(|err| {
                AppErr::custom(format!(
                    "failed to parse CoinGecko price for {}: {err}",
                    market.name
                ))
            })?;

            Ok(AssetRecord {
                name: market.name,
                symbol: market.symbol,
                price,
            })
        })
        .collect()
}
