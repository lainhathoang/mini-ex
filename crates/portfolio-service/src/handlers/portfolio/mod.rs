use axum::{Router, routing::get};

use crate::extractors::state::AppState;

mod get;

pub fn routes() -> Router<AppState> {
    Router::new().route("/portfolio/{user_id}", get(get::handler))
}
