use axum::{Router, routing::get};
use chrono::NaiveDateTime;
use database::{
    entities::orders,
    enums::{OrderSide, OrderStatus, RejectReason},
};
use serde::Serialize;

use crate::extractors::state::AppState;

mod create;
mod execution;
mod get;
mod list;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders", get(list::handler).post(create::handler))
        .route("/orders/{order_id}", get(get::handler))
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub user_id: String,
    pub client_order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: String,
    pub price: Option<String>,
    pub status: OrderStatus,
    pub reject_reason: Option<RejectReason>,
    pub created_at: NaiveDateTime,
}

impl From<orders::Model> for OrderResponse {
    fn from(order: orders::Model) -> Self {
        Self {
            id: order.id.to_string(),
            user_id: order.user_id.to_string(),
            client_order_id: order.client_order_id,
            symbol: order.symbol,
            side: order.side,
            quantity: order.quantity.to_string(),
            price: order.price.map(|price| price.to_string()),
            status: order.status,
            reject_reason: order.reject_reason,
            created_at: order.created_at,
        }
    }
}
