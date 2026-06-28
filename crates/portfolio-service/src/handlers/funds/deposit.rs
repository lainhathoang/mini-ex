use std::str::FromStr;

use axum::{Json, extract::State};
use database::{
    repositories,
    sea_orm::{DatabaseConnection, TransactionTrait, prelude::Decimal, prelude::Uuid},
};
use serde::{Deserialize, Serialize};

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

#[derive(Deserialize)]
pub struct DepositRequest {
    pub amount: String,
}

#[derive(Serialize)]
pub struct DepositResponse {
    pub cash_balance: String,
}

/// `POST /funds/deposit` — testing-only USD top-up for the authenticated user.
///
/// Credits the amount to the caller's `cash_balance` (taken from the JWT, never
/// the body) and returns the updated balance. The read-modify-write runs under
/// a `SELECT ... FOR UPDATE` row lock so concurrent deposits cannot lose
/// updates.
pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
    Json(payload): Json<DepositRequest>,
) -> HttpResult<Json<DepositResponse>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpException::internal("invalid user id in token"))?;

    let amount = Decimal::from_str(payload.amount.trim())
        .map_err(|_| HttpException::bad_request("amount must be a valid number"))?;
    if amount <= Decimal::ZERO {
        return Err(HttpException::bad_request("amount must be greater than 0"));
    }

    let txn = db.begin().await.map_err(HttpException::internal)?;

    let user = repositories::users::find_by_id_for_update(&txn, user_id)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    let new_balance = user
        .cash_balance
        .checked_add(amount)
        .ok_or_else(|| HttpException::internal("cash balance overflow"))?;
    repositories::users::update_cash_balance(&txn, user_id, new_balance).await?;

    txn.commit().await.map_err(HttpException::internal)?;

    Ok(Json(DepositResponse {
        cash_balance: new_balance.to_string(),
    }))
}
