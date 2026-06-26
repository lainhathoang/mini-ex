use std::{str::FromStr, time::Duration};

use database::{models::assets::AssetRecord, repositories, sea_orm::prelude::Decimal};
use serde::Deserialize;
use shared::{
    env::Env,
    result::{AppErr, Rs},
};
use tokio::time::MissedTickBehavior;

const COINGECKO_MARKETS_URL: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1";
const SYNC_INTERVAL: Duration = Duration::from_secs(5 * 60);

#[derive(Debug, Deserialize)]
struct CoinGeckoMarket {
    name: String,
    symbol: String,
    current_price: serde_json::Number,
}

#[tokio::main]
async fn main() -> Rs<()> {
    shared::tracing::subscribe();
    shared::env::load();

    let db_url = shared::env::read(Env::DatabaseUrl)?;
    let api_key = shared::env::read(Env::CoingeckoApiKey)?;

    let db = database::establish_connection(&db_url).await?;
    let client = reqwest::Client::new();

    tracing::info!("Connected to DB");

    let mut interval = tokio::time::interval(SYNC_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        if let Err(err) = sync_assets(&client, &db, &api_key).await {
            tracing::error!("market-service sync failed, {}", err.to_string());
        }
    }
}

async fn sync_assets(
    client: &reqwest::Client,
    db: &database::sea_orm::DatabaseConnection,
    api_key: &str,
) -> Rs<()> {
    let assets = fetch_assets(client, api_key).await?;
    repositories::assets::upsert_many(db, &assets).await?;

    tracing::info!("Synced {} assets from CoinGecko", assets.len());

    Ok(())
}

async fn fetch_assets(client: &reqwest::Client, api_key: &str) -> Rs<Vec<AssetRecord>> {
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
