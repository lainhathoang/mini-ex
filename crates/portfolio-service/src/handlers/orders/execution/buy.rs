use axum::{Json, http::StatusCode};
use database::{
    enums::{OrderSide, OrderStatus, RejectReason},
    repositories,
    sea_orm::TransactionTrait,
};
use shared::util::checked_mul;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::state::AppState,
};

use super::{
    OrderCtx, OrderResponse, commit_rejected, new_order, price_order, reject_locked,
    replay_under_lock,
};

pub async fn execute_buy(
    state: &AppState,
    ctx: &OrderCtx<'_>,
) -> HttpResult<(StatusCode, Json<OrderResponse>)> {
    let price = match price_order(state, ctx.symbol).await {
        Ok(price) => price,
        Err(reason) => return reject_locked(state, ctx, OrderSide::Buy, reason).await,
    };
    let total = checked_mul(price, ctx.quantity)?;

    let txn = state.db.begin().await.map_err(HttpException::internal)?;

    let user = repositories::users::find_by_id_for_update(&txn, ctx.user_id)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    if let Some(existing) = replay_under_lock(&txn, ctx).await? {
        return Ok((StatusCode::OK, Json(existing)));
    }

    if user.cash_balance < total {
        return commit_rejected(txn, ctx, OrderSide::Buy, RejectReason::InsufficientCash).await;
    }

    let new_cash = user
        .cash_balance
        .checked_sub(total)
        .ok_or_else(|| HttpException::internal("cash balance underflow"))?;
    repositories::users::update_cash_balance(&txn, ctx.user_id, new_cash).await?;
    repositories::asset_balances::add_quantity(&txn, ctx.user_id, ctx.symbol, ctx.quantity).await?;

    let order = repositories::orders::create(
        &txn,
        new_order(
            ctx,
            OrderSide::Buy,
            OrderStatus::Executed,
            Some(price),
            None,
        ),
    )
    .await?;

    txn.commit().await.map_err(HttpException::internal)?;
    Ok((StatusCode::CREATED, Json(order.into())))
}
