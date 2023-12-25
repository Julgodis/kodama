use axum::{
    response::{IntoResponse, Response},
    Json,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rusqlite error")]
    Rusqlite(#[from] rusqlite::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("Api Error: {0}")]
    ApiError(#[from] ApiError),
    #[error("Utf8 Error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Serde Json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);

        match self {
            Error::ApiError(x) => {
                (axum::http::StatusCode::BAD_REQUEST, Json(x.json())).into_response()
            }
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid project name")]
    InvalidProjectName,
    #[error("invalid service name")]
    InvalidServiceName,
    #[error("project not found")]
    ProjectNotFound,
    #[error("service not found")]
    ServiceNotFound(String),
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("record not found")]
    RecordNotFound,
}

impl ApiError {
    pub fn json(&self) -> kodama_api::ErrorResponse {
        tracing::error!("{:?}", self);
        kodama_api::ErrorResponse {
            code: 10001,
            message: "invalid project name".to_string(),
        }
    }
}
