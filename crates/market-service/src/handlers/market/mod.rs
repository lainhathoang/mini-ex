use axum::{Router, routing::get};

use crate::extractors::state::AppState;

mod price;
mod prices;
mod symbols;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/symbols", get(symbols::handler))
        .route("/prices", get(prices::handler))
        .route("/prices/{symbol}", get(price::handler))
}
