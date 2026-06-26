use std::borrow::Cow;

type Location = &'static core::panic::Location<'static>;

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("I/O: {src}")]
    Io {
        src: std::io::Error,
        location: Location,
    },

    #[error("Custom: {message}")]
    Custom {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("ParseInt: {src}")]
    ParseInt {
        src: std::num::ParseIntError,
        location: Location,
    },

    #[error("Database: {src}")]
    Database {
        src: sea_orm::error::DbErr,
        location: Location,
    },

    #[error("ReadEnv: {src}")]
    ReadEnv {
        src: std::env::VarError,
        location: Location,
    },

    #[error("ParseUrl: {src}")]
    ParseUrl {
        src: url::ParseError,
        location: Location,
    },

    #[error("ParseUrl: {src}")]
    ParseUri {
        src: hyper::http::uri::InvalidUri,
        location: Location,
    },

    #[error("HttpClient: {src}")]
    HttpClient {
        src: reqwest::Error,
        location: Location,
    },
}

macro_rules! impl_from_tracked {
    ($source_type:ty, $variant:ident) => {
        impl From<$source_type> for AppErr {
            #[track_caller]
            fn from(src: $source_type) -> Self {
                Self::$variant {
                    src,
                    location: core::panic::Location::caller(),
                }
            }
        }
    };
}

impl_from_tracked!(std::io::Error, Io);
impl_from_tracked!(std::num::ParseIntError, ParseInt);
impl_from_tracked!(sea_orm::error::DbErr, Database);
impl_from_tracked!(std::env::VarError, ReadEnv);
impl_from_tracked!(url::ParseError, ParseUrl);
impl_from_tracked!(hyper::http::uri::InvalidUri, ParseUri);
impl_from_tracked!(reqwest::Error, HttpClient);

pub type Rs<T> = Result<T, AppErr>;

impl AppErr {
    pub fn location(&self) -> Location {
        match self {
            AppErr::Custom { location, .. } => location,
            AppErr::Database { location, .. } => location,
            AppErr::Io { location, .. } => location,
            AppErr::ParseInt { location, .. } => location,
            AppErr::ReadEnv { location, .. } => location,
            AppErr::ParseUrl { location, .. } => location,
            AppErr::ParseUri { location, .. } => location,
            AppErr::HttpClient { location, .. } => location,
        }
    }

    pub fn trace<C: AsRef<str>>(&self, ctx: C) {
        tracing::error!("{} >> {}\nTrace: {}", ctx.as_ref(), self, self.location());
    }

    #[track_caller]
    pub fn custom<E: Into<Cow<'static, str>>>(message: E) -> AppErr {
        AppErr::Custom {
            message: message.into(),
            location: core::panic::Location::caller(),
        }
    }
}
