pub mod metric;
pub mod project;
pub mod query;
pub mod record;
pub mod service;
pub mod sql_trace;

mod database;
mod database_connection;
mod database_query;
mod from_row;
pub use database::*;
pub use database_connection::*;
pub use database_query::*;
pub use from_row::*;
use metric::Timestamp;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Self::Rusqlite(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Rusqlite(error) => write!(f, "rusqlite error: {}", error),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    Metric(Metric),
    Record(Record),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    pub project_name: String,
    pub service_name: String,
    pub record_name: String,

    pub group_by: String,
    pub timestamp: Option<Timestamp>,
    pub execution_time_us: u64,
    pub error: i64, // if >0 then error
}

mod client;
pub use client::*;

pub use rusqlite::params;

#[cfg(feature = "admin")]
mod admin_client;
#[cfg(feature = "admin")]
pub use admin_client::*;
use metric::Metric;
