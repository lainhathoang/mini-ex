//! CoinGecko integration: background service that periodically fetches market
//! data and upserts it into the database.

mod fetch_assets;
mod sync_assets;

pub use sync_assets::run;
