use axum::{Json, extract::State};
use chrono::NaiveDateTime;
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Auth,
};

#[derive(Serialize)]
pub struct Response {
    id: String,
    username: String,
    created_at: NaiveDateTime,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Auth(claims): Auth,
) -> HttpResult<Json<Response>> {
    let user = repositories::users::find_by_username(&db, &claims.username)
        .await?
        .ok_or_else(|| HttpException::internal("user not found"))?;

    let response = Response {
        id: user.id.to_string(),
        username: claims.username.to_string(),
        created_at: user.created_at,
    };

    Ok(Json(response))
}
