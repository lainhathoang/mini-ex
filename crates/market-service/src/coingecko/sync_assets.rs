use std::time::Duration;

use database::{repositories, sea_orm::DatabaseConnection};
use shared::result::Rs;
use tokio::time::MissedTickBehavior;

use super::fetch_assets::fetch_assets;

const SYNC_INTERVAL: Duration = Duration::from_secs(10 * 60);

pub async fn run(db: DatabaseConnection, api_key: String) {
    let client = reqwest::Client::new();

    let mut interval = tokio::time::interval(SYNC_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        if let Err(err) = sync_assets(&client, &db, &api_key).await {
            tracing::error!("market-service sync failed, {}", err.to_string());
        }
    }
}

async fn sync_assets(client: &reqwest::Client, db: &DatabaseConnection, api_key: &str) -> Rs<()> {
    let assets = fetch_assets(client, api_key).await?;
    repositories::assets::upsert_many(db, &assets).await?;

    tracing::info!("Synced {} assets from CoinGecko", assets.len());

    Ok(())
}
