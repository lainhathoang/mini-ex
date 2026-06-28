use axum::{
    Json,
    extract::{Path, State},
};
use database::{repositories, sea_orm::DatabaseConnection, sea_orm::prelude::Uuid};

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

use super::OrderResponse;

/// `GET /orders/{order_id}` — fetch a single order owned by the caller.
///
/// Unknown ids return `404`; orders owned by another user return `403` so
/// existence is only revealed to the owner.
pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
    Path(order_id): Path<String>,
) -> HttpResult<Json<OrderResponse>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpException::internal("invalid user id in token"))?;

    let order_id = Uuid::parse_str(&order_id)
        .map_err(|_| HttpException::bad_request("invalid order id format"))?;

    let order = repositories::orders::find_by_id(&db, order_id)
        .await?
        .ok_or_else(|| HttpException::not_found("order not found"))?;

    if order.user_id != user_id {
        return Err(HttpException::forbidden("cannot access another user's order"));
    }

    Ok(Json(order.into()))
}
