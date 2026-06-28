use axum::{Json, http::StatusCode};
use database::{
    enums::{OrderSide, OrderStatus, RejectReason},
    repositories,
    repositories::orders::NewOrder,
    sea_orm::{DatabaseTransaction, TransactionTrait, prelude::Decimal, prelude::Uuid},
};

use crate::{
    common::market_client::{self, PriceError},
    exception::{HttpException, HttpResult},
    extractors::state::AppState,
};

use super::OrderResponse;

pub mod buy;
pub mod sell;

/// Per-request order context built once in the dispatcher and threaded through
/// the buy/sell flows and the shared helpers below.
pub struct OrderCtx<'a> {
    pub user_id: Uuid,
    pub client_order_id: &'a str,
    pub symbol: &'a str,
    pub quantity: Decimal,
}

// --- Helpers shared by the buy/sell submodules ---
//
// These stay private to the `execution` module: the `buy` and `sell` children
// reach them directly, and nothing outside execution needs them.

/// Replay check performed while holding the user-row lock: handles a concurrent
/// retry that committed first, so it is returned instead of double-executing.
///
/// Race-safety relies on READ COMMITTED (the connection default): this SELECT
/// takes a fresh snapshot after the lock is acquired, so it sees an order a
/// concurrent request committed just before releasing the lock.
async fn replay_under_lock(
    txn: &DatabaseTransaction,
    ctx: &OrderCtx<'_>,
) -> HttpResult<Option<OrderResponse>> {
    Ok(
        repositories::orders::find_by_client_order_id(txn, ctx.user_id, ctx.client_order_id)
            .await?
            .map(Into::into),
    )
}

/// Fetches the order price, mapping market-service failures to reject reasons.
async fn price_order(state: &AppState, symbol: &str) -> Result<Decimal, RejectReason> {
    market_client::fetch_price(&state.http_client, &state.market_service_url, symbol)
        .await
        .map_err(|err| match err {
            PriceError::Unavailable => RejectReason::MarketServiceUnavailable,
            PriceError::InvalidSymbol => RejectReason::InvalidSymbol,
        })
}

/// Builds a [`NewOrder`] from the request context plus per-outcome fields.
fn new_order<'a>(
    ctx: &OrderCtx<'a>,
    side: OrderSide,
    status: OrderStatus,
    price: Option<Decimal>,
    reject_reason: Option<RejectReason>,
) -> NewOrder<'a> {
    NewOrder {
        user_id: ctx.user_id,
        client_order_id: ctx.client_order_id,
        symbol: ctx.symbol,
        side,
        quantity: ctx.quantity,
        status,
        price,
        reject_reason,
    }
}

/// Rejects an order that failed before the execution transaction (price failure
/// or preliminary asset shortfall). Takes the user-row lock and replays a
/// concurrent retry so a double-submit cannot collide on the idempotency key.
async fn reject_locked(
    state: &AppState,
    ctx: &OrderCtx<'_>,
    side: OrderSide,
    reason: RejectReason,
) -> HttpResult<(StatusCode, Json<OrderResponse>)> {
    let txn = state.db.begin().await.map_err(HttpException::internal)?;

    repositories::users::find_by_id_for_update(&txn, ctx.user_id)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    if let Some(existing) = replay_under_lock(&txn, ctx).await? {
        return Ok((StatusCode::OK, Json(existing)));
    }

    commit_rejected(txn, ctx, side, reason).await
}

/// Persists a `REJECTED` order (no balance changes) and commits, returning it
/// with `201`. The caller must already hold the user-row lock and have run the
/// replay check.
async fn commit_rejected(
    txn: DatabaseTransaction,
    ctx: &OrderCtx<'_>,
    side: OrderSide,
    reason: RejectReason,
) -> HttpResult<(StatusCode, Json<OrderResponse>)> {
    let order = repositories::orders::create(
        &txn,
        new_order(ctx, side, OrderStatus::Rejected, None, Some(reason)),
    )
    .await?;
    txn.commit().await.map_err(HttpException::internal)?;
    Ok((StatusCode::CREATED, Json(order.into())))
}
