use axum::{Router, response::Html, routing::get};
use shared::{
    env::{Env, read},
    result::Rs,
};
use tower_http::cors::CorsLayer;

use crate::extractors::state::AppState;

mod coingecko;
mod exception;
mod extractors;
mod handlers;

#[tokio::main]
async fn main() -> Rs<()> {
    shared::tracing::subscribe();
    shared::env::load();

    let api_key = shared::env::read(Env::CoingeckoApiKey)?;
    let state = AppState::new().await?;

    tracing::info!("Connected to DB");

    // Run the CoinGecko sync as a background service.
    tokio::spawn(coingecko::run(state.db.clone(), api_key));

    let app = Router::new()
        .route("/", get(async || "Hello from market-service!"))
        .route(
            "/docs/openapi.yml",
            get(async || include_str!("../docs/openapi.yml")),
        )
        .route(
            "/swagger",
            get(async || Html(include_str!("../docs/swagger.html"))),
        )
        .route(
            "/scalar",
            get(async || Html(include_str!("../docs/scalar.html"))),
        )
        .merge(handlers::market::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = read(Env::MarketServicePort)?;
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Market service is running {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
