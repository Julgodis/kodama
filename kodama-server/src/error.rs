#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("kodama error")]
    KodamaError(#[from] kodama_internal::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("serde json Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
