use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::exception::HttpResult;

#[derive(Serialize)]
pub struct Response {
    symbols: Vec<String>,
}

/// `GET /symbols` — lists every tracked asset symbol.
pub async fn handler(State(db): State<DatabaseConnection>) -> HttpResult<Json<Response>> {
    let assets = repositories::assets::find_all(&db).await?;

    let symbols = assets.into_iter().map(|asset| asset.symbol).collect();

    Ok(Json(Response { symbols }))
}
