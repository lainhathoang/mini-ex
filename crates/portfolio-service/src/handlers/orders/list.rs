use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection, sea_orm::prelude::Uuid};

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

use super::OrderResponse;

/// `GET /orders` — list the authenticated user's orders, newest first.
pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
) -> HttpResult<Json<Vec<OrderResponse>>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| HttpException::internal("invalid user id in token"))?;

    let orders = repositories::orders::find_by_user(&db, user_id).await?;

    Ok(Json(orders.into_iter().map(Into::into).collect()))
}
