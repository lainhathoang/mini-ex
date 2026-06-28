use std::time::Duration;

use axum::extract::FromRef;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};

/// Total budget for a market-service price lookup. Order placement is
/// synchronous, so a hung market-service must fail fast rather than block.
const MARKET_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub http_client: reqwest::Client,
    pub market_service_url: String,
}

impl AppState {
    pub async fn new() -> Rs<AppState> {
        let db_url = shared::env::read(Env::PortfolioDatabaseUrl)?;
        let db = database::establish_connection(&db_url).await?;
        let market_service_url = shared::env::read(Env::MarketServiceUrl)?;
        let http_client = reqwest::Client::builder()
            .timeout(MARKET_REQUEST_TIMEOUT)
            .build()?;
        Ok(Self {
            db,
            http_client,
            market_service_url,
        })
    }
}
