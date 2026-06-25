use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    exception::{HttpException, HttpResult},
    extractors::validator::ValidatedPayload,
};

use super::{CredentialsRequest, validate_trimmed_credentials};

#[derive(Serialize)]
pub struct SignUpResponse {
    id: String,
    username: String,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    ValidatedPayload(payload): ValidatedPayload<CredentialsRequest>,
) -> HttpResult<(StatusCode, Json<SignUpResponse>)> {
    let username = payload.username.trim();
    let password = payload.password.trim();

    validate_trimmed_credentials(username, password)?;

    if repositories::users::find_by_username(&db, username)
        .await?
        .is_some()
    {
        return Err(HttpException::bad_request("username already taken"));
    }

    let password_hash = hash_password(password)?;
    let user = repositories::users::create_user(&db, username, &password_hash).await?;

    Ok((
        StatusCode::CREATED,
        Json(SignUpResponse {
            id: user.id.to_string(),
            username: user.username,
        }),
    ))
}

fn hash_password(password: &str) -> HttpResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(HttpException::internal)
        .map(|password_hash| password_hash.to_string())
}
