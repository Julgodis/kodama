use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{
    types::{FromSql, FromSqlResult, ValueRef},
    ToSql,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushRequest {
    pub project_name: String,
    pub service_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub metric_timestamp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushResponse {
    pub metric_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metric {
    pub project_name: String,
    pub service_name: String,
    pub metric_name: String,

    pub metric_timestamp: Option<Timestamp>,
    pub metric_value: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timestamp {
    pub microseconds: u64,
}

impl Timestamp {
    pub fn now() -> Option<Self> {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).ok()?;
        let microseconds = duration.as_micros() as u64;
        Some(Self { microseconds })
    }
}

impl ToSql for Timestamp {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.microseconds as i64),
        ))
    }
}

impl FromSql for Timestamp {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(Self {
            microseconds: value.as_i64()? as u64,
        })
    }
}
