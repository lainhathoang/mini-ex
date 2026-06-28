use axum::{Router, routing::post};
use validator::Validate;

use crate::exception::{HttpException, HttpResult};
use crate::extractors::state::AppState;

mod sign_in;
mod sign_out;
mod sign_up;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/sign-up", post(sign_up::handler))
        .route("/auth/sign-in", post(sign_in::handler))
        .route("/auth/sign-out", post(sign_out::handler))
}

#[derive(serde::Deserialize, Validate)]
pub struct CredentialsRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub fn validate_trimmed_credentials(username: &str, password: &str) -> HttpResult<()> {
    if username.is_empty() {
        return Err(HttpException::bad_request("username cannot be empty"));
    }

    if password.is_empty() {
        return Err(HttpException::bad_request("password cannot be empty"));
    }

    let username_len = username.chars().count();
    if !(3..=32).contains(&username_len) {
        return Err(HttpException::bad_request(
            "username length must be between 3 and 32",
        ));
    }

    if password.chars().count() < 8 {
        return Err(HttpException::bad_request(
            "password length must be at least 8",
        ));
    }

    Ok(())
}
