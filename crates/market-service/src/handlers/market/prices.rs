use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::exception::HttpResult;

#[derive(Serialize)]
pub struct Price {
    symbol: String,
    name: String,
    price: String,
}

#[derive(Serialize)]
pub struct Response {
    prices: Vec<Price>,
}

/// `GET /prices` — lists every tracked asset with its latest price.
pub async fn handler(State(db): State<DatabaseConnection>) -> HttpResult<Json<Response>> {
    let assets = repositories::assets::find_all(&db).await?;

    let prices = assets
        .into_iter()
        .map(|asset| Price {
            symbol: asset.symbol,
            name: asset.name,
            price: asset.price.to_string(),
        })
        .collect();

    Ok(Json(Response { prices }))
}
