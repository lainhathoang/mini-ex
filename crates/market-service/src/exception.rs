use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use shared::result::AppErr;

type Location = &'static core::panic::Location<'static>;

#[derive(thiserror::Error, Debug)]
pub enum HttpException {
    #[error("BadRequest: {msg}")]
    BadRequest {
        msg: Cow<'static, str>,
        location: Location,
    },

    #[error("NotFound: {msg}")]
    NotFound {
        msg: Cow<'static, str>,
        location: Location,
    },

    #[error("msg: {msg}")]
    Internal {
        msg: Cow<'static, str>,
        location: Location,
    },

    // AppErr already carries its own location — no need to duplicate
    #[error(transparent)]
    App(#[from] AppErr),
}

pub type HttpResult<A> = Result<A, HttpException>;

impl HttpException {
    fn location(&self) -> Location {
        match self {
            Self::BadRequest { location, .. } => location,
            Self::NotFound { location, .. } => location,
            Self::Internal { location, .. } => location,
            Self::App(error) => error.location(),
        }
    }

    fn trace(&self) {
        tracing::error!("{}\nTrace: {}", self, self.location());
    }

    #[track_caller]
    #[allow(dead_code)]
    pub fn internal<E: ToString>(error: E) -> Self {
        Self::Internal {
            msg: error.to_string().into(),
            location: core::panic::Location::caller(),
        }
    }

    #[track_caller]
    #[allow(dead_code)]
    pub fn bad_request<E: Into<Cow<'static, str>>>(error: E) -> Self {
        Self::BadRequest {
            msg: error.into(),
            location: core::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn not_found<E: Into<Cow<'static, str>>>(error: E) -> Self {
        Self::NotFound {
            msg: error.into(),
            location: core::panic::Location::caller(),
        }
    }
}

impl IntoResponse for HttpException {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => {
                self.trace();
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = Json(json!({
            "code": status_code.as_u16(),
            "msg": self.to_string(),
        }));

        (status_code, body).into_response()
    }
}
