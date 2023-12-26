#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("rusqlite error")]
    Rusqlite(#[from] rusqlite::Error),
    #[error("api error: {0}")]
    ApiError(#[from] ApiError),
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

pub type Result<T> = std::result::Result<T, Error>;
