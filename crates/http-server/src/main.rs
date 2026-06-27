use axum::{Router, response::Html, routing::get};
use shared::{
    env::{Env, read},
    result::Rs,
};
use tower_http::cors::CorsLayer;

use crate::extractors::state::AppState;

mod common;
mod exception;
mod extractors;
mod handlers;

#[tokio::main]
async fn main() -> Rs<()> {
    shared::tracing::subscribe();
    shared::env::load();

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/", get(async || "hello !"))
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
        .merge(handlers::auth::routes())
        .merge(handlers::users::routes())
        .merge(handlers::ws::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = read(Env::HttpServerPort)?;
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server is running {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
