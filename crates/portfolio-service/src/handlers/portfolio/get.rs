use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection, sea_orm::prelude::Uuid};
use serde::Serialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

use axum::extract::Path;

#[derive(Serialize)]
pub struct AssetBalanceDto {
    pub symbol: String,
    pub quantity: String,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    pub user_id: String,
    pub username: String,
    pub cash_balance: String,
    pub assets: Vec<AssetBalanceDto>,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
    Path(user_id_str): Path<String>,
) -> HttpResult<Json<PortfolioResponse>> {
    let user_uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpException::internal("invalid user id in token"))?;

    let requested_uuid = Uuid::parse_str(&user_id_str)
        .map_err(|_| HttpException::bad_request("invalid user id format"))?;

    if user_uuid != requested_uuid {
        return Err(HttpException::forbidden(
            "cannot access another user's portfolio",
        ));
    }

    let user = repositories::users::find_by_id(&db, user_uuid)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    let balances = repositories::asset_balances::find_by_user(&db, user_uuid).await?;

    Ok(Json(PortfolioResponse {
        user_id: user.id.to_string(),
        username: user.username,
        cash_balance: user.cash_balance.to_string(),
        assets: balances
            .into_iter()
            .map(|b| AssetBalanceDto {
                symbol: b.symbol,
                quantity: b.quantity.to_string(),
            })
            .collect(),
    }))
}
