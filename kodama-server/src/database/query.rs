use crate::{
    kodama::Database,
    value::{Column, Param, Row},
    Result,
};
use axum::Json;
use rusqlite::{
    types::{self, FromSql, ToSqlOutput},
    ToSql,
};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Request {
    database: Uuid,
    statement: String,
    params: Vec<Param>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response {
    database: Uuid,
    columns: Vec<Column>,
    rows: Vec<Row>,
}

pub async fn handler(Json(input): Json<Request>) -> Result<Json<Response>> {
    tracing::debug!("query: {:?}", input);

    let db = Database::myself(&input.database)?;
    let result = db.query(&input.statement, &input.params)?;

    Ok(Json(Response {
        database: input.database,
        columns: result.columns,
        rows: result.rows,
    }))
}
