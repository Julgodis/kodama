use axum::Json;
use uuid::Uuid;

use crate::kodama::Database;
use crate::value::Param;
use crate::Result;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Request {
    database: Uuid,
    statement: String,
    params: Vec<Param>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response {
    database: Uuid,
}

pub async fn handler(Json(input): Json<Request>) -> Result<Json<Response>> {
    tracing::debug!("execute: {:?}", input);

    let db = Database::myself(&input.database)?;
    let _ = db.execute(&input.statement, &input.params)?;

    Ok(Json(Response {
        database: input.database,
    }))
}
