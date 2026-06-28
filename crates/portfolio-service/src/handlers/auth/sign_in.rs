use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    common::jwt,
    exception::{HttpException, HttpResult},
    extractors::validator::ValidatedPayload,
};

use super::{CredentialsRequest, validate_trimmed_credentials};

#[derive(Serialize)]
pub struct SignInResponse {
    token: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    ValidatedPayload(payload): ValidatedPayload<CredentialsRequest>,
) -> HttpResult<Json<SignInResponse>> {
    let username = payload.username.trim();
    let password = payload.password.trim();

    validate_trimmed_credentials(username, password)?;

    let user = repositories::users::find_by_username(&db, username)
        .await?
        .ok_or_else(|| HttpException::unauthorized("invalid credentials"))?;

    verify_password(password, &user.password_hash)?;

    let token = jwt::sign(&user.id.to_string(), &user.username)?;

    Ok(Json(SignInResponse { token }))
}

fn verify_password(password: &str, password_hash: &str) -> HttpResult<()> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(HttpException::internal)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| HttpException::unauthorized("invalid credentials"))
}
