use axum::{
    Json,
    extract::{Path, State},
};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::exception::{HttpException, HttpResult};

#[derive(Serialize)]
pub struct Response {
    symbol: String,
    name: String,
    price: String,
}

/// `GET /prices/{symbol}` — returns the latest price for a single asset.
///
/// Symbols are stored lowercase (as returned by CoinGecko), so the path
/// parameter is normalized before lookup. Responds `404` when unknown.
pub async fn handler(
    State(db): State<DatabaseConnection>,
    Path(symbol): Path<String>,
) -> HttpResult<Json<Response>> {
    let symbol = symbol.to_lowercase();

    let asset = repositories::assets::find_by_symbol(&db, &symbol)
        .await?
        .ok_or_else(|| HttpException::not_found(format!("symbol not found: {symbol}")))?;

    Ok(Json(Response {
        symbol: asset.symbol,
        name: asset.name,
        price: asset.price.to_string(),
    }))
}
