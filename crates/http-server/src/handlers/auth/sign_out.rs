use axum::http::StatusCode;

use crate::{exception::HttpResult, extractors::auth::Auth};

pub async fn handler(Auth(_claims): Auth) -> HttpResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
