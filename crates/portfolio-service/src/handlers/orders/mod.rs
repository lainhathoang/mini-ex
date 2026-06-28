use axum::{Router, routing::get};
use chrono::NaiveDateTime;
use database::{
    entities::orders,
    enums::{OrderSide, OrderStatus, RejectReason},
};
use serde::Serialize;

use crate::extractors::state::AppState;

mod create;
mod get;

pub fn routes() -> Router<AppState> {
    Router::new().route("/orders/{order_id}", get(get::handler))
}
// .route("/orders", post(create::handler))

/// API representation of an [`orders::Model`].
///
/// Decimal values are rendered as strings to avoid any floating-point
/// round-tripping, and the DB enums are mapped to their canonical
/// uppercase wire values (`BUY`/`SELL`, `EXECUTED`/`REJECTED`, ...).
#[derive(Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: String,
    pub price: Option<String>,
    pub status: String,
    pub reject_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<orders::Model> for OrderResponse {
    fn from(order: orders::Model) -> Self {
        Self {
            id: order.id.to_string(),
            user_id: order.user_id.to_string(),
            client_order_id: order.client_order_id,
            symbol: order.symbol,
            side: side_to_str(&order.side).to_owned(),
            quantity: order.quantity.to_string(),
            price: order.price.map(|price| price.to_string()),
            status: status_to_str(&order.status).to_owned(),
            reject_reason: order.reject_reason.as_ref().map(reject_reason_to_str),
            created_at: order.created_at,
        }
    }
}

fn side_to_str(side: &OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "BUY",
        OrderSide::Sell => "SELL",
    }
}

fn status_to_str(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::Created => "CREATED",
        OrderStatus::Executed => "EXECUTED",
        OrderStatus::Rejected => "REJECTED",
    }
}

fn reject_reason_to_str(reason: &RejectReason) -> String {
    match reason {
        RejectReason::InsufficientCash => "INSUFFICIENT_CASH",
        RejectReason::InsufficientAsset => "INSUFFICIENT_ASSET",
        RejectReason::MarketServiceUnavailable => "MARKET_SERVICE_UNAVAILABLE",
        RejectReason::InvalidSymbol => "INVALID_SYMBOL",
        RejectReason::InvalidQuantity => "INVALID_QUANTITY",
    }
    .to_owned()
}
