use axum::{Json, http::StatusCode};
use database::{
    enums::{OrderSide, OrderStatus, RejectReason},
    repositories,
    sea_orm::{ConnectionTrait, TransactionTrait, prelude::Decimal, prelude::Uuid},
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

pub async fn execute_sell(
    state: &AppState,
    ctx: &OrderCtx<'_>,
) -> HttpResult<(StatusCode, Json<OrderResponse>)> {
    if held_quantity(&state.db, ctx.user_id, ctx.symbol).await? < ctx.quantity {
        return reject_locked(state, ctx, OrderSide::Sell, RejectReason::InsufficientAsset).await;
    }

    let price = match price_order(state, ctx.symbol).await {
        Ok(price) => price,
        Err(reason) => return reject_locked(state, ctx, OrderSide::Sell, reason).await,
    };
    let proceeds = checked_mul(price, ctx.quantity)?;

    let txn = state.db.begin().await.map_err(HttpException::internal)?;

    let user = repositories::users::find_by_id_for_update(&txn, ctx.user_id)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    if let Some(existing) = replay_under_lock(&txn, ctx).await? {
        return Ok((StatusCode::OK, Json(existing)));
    }

    if held_quantity(&txn, ctx.user_id, ctx.symbol).await? < ctx.quantity {
        return commit_rejected(txn, ctx, OrderSide::Sell, RejectReason::InsufficientAsset).await;
    }

    repositories::asset_balances::sub_quantity(&txn, ctx.user_id, ctx.symbol, ctx.quantity).await?;
    let new_cash = user
        .cash_balance
        .checked_add(proceeds)
        .ok_or_else(|| HttpException::internal("cash balance overflow"))?;
    repositories::users::update_cash_balance(&txn, ctx.user_id, new_cash).await?;

    let order = repositories::orders::create(
        &txn,
        new_order(
            ctx,
            OrderSide::Sell,
            OrderStatus::Executed,
            Some(price),
            None,
        ),
    )
    .await?;

    txn.commit().await.map_err(HttpException::internal)?;
    Ok((StatusCode::CREATED, Json(order.into())))
}

// user + symbol => quantity
async fn held_quantity(
    conn: &impl ConnectionTrait,
    user_id: Uuid,
    symbol: &str,
) -> HttpResult<Decimal> {
    Ok(
        repositories::asset_balances::find_by_user_and_symbol(conn, user_id, symbol)
            .await?
            .map(|balance| balance.quantity)
            .unwrap_or(Decimal::ZERO),
    )
}
