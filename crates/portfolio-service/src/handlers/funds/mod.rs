use axum::{Router, routing::post};

use crate::extractors::state::AppState;

mod deposit;

pub fn routes() -> Router<AppState> {
    Router::new().route("/funds/deposit", post(deposit::handler))
}
