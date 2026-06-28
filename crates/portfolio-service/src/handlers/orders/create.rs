use std::str::FromStr;

use axum::{Json, extract::State, http::StatusCode};
use database::{
    enums::OrderSide,
    repositories,
    sea_orm::{prelude::Decimal, prelude::Uuid},
};
use serde::Deserialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::{auth::Auth, state::AppState},
    handlers::orders::execution::{OrderCtx, buy::execute_buy, sell::execute_sell},
};

use super::OrderResponse;

const MAX_CLIENT_ORDER_ID_LEN: usize = 100;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: String,
    pub client_order_id: String,
}

/// `POST /orders` — place and synchronously execute a market order.
///
/// The acting user is always taken from the JWT, never the request body. A
/// required `client_order_id` makes placement idempotent: retries (double-click,
/// network re-send) replay the original order with `200` instead of placing a
/// duplicate. Fresh orders execute atomically inside a transaction that locks
/// the user row, so concurrent requests cannot lose updates or double-execute.
///
/// Validation and idempotency live here; the side-specific execution lives in
/// the [`super::buy`] and [`super::sell`] modules, over shared helpers in
/// [`super::shared`].
pub async fn handler(
    State(state): State<AppState>,
    Auth(claims): Auth,
    Json(payload): Json<CreateOrderRequest>,
) -> HttpResult<(StatusCode, Json<OrderResponse>)> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpException::internal("invalid user id in token"))?;

    // Idempotency key is validated first: a retry must replay the original order
    // even if the rest of the payload differs or is malformed.
    let client_order_id = payload.client_order_id.trim();
    if client_order_id.is_empty() {
        return Err(HttpException::bad_request("client_order_id is required"));
    }
    if client_order_id.chars().count() > MAX_CLIENT_ORDER_ID_LEN {
        return Err(HttpException::bad_request("client_order_id is too long"));
    }

    // Fast replay path: a previously processed order with this key is returned
    // as-is, without re-pricing or re-executing.
    if let Some(existing) =
        repositories::orders::find_by_client_order_id(&state.db, user_id, client_order_id).await?
    {
        return Ok((StatusCode::OK, Json(existing.into())));
    }

    // --- Request validation (pre-order; failures never create an order) ---
    let symbol = payload.symbol.trim().to_lowercase();
    if symbol.is_empty() {
        return Err(HttpException::bad_request("symbol is required"));
    }

    let side = payload.side;

    let quantity = Decimal::from_str(payload.quantity.trim())
        .map_err(|_| HttpException::bad_request("quantity must be a valid number"))?;
    if quantity <= Decimal::ZERO {
        return Err(HttpException::bad_request(
            "quantity must be greater than 0",
        ));
    }

    let ctx = OrderCtx {
        user_id,
        client_order_id,
        symbol: &symbol,
        quantity,
    };

    match side {
        OrderSide::Buy => execute_buy(&state, &ctx).await,
        OrderSide::Sell => execute_sell(&state, &ctx).await,
    }
}
