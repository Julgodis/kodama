use axum::Json;
use uuid::Uuid;

use crate::kodama::Kodama;
use crate::Result;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Request {
    project: Uuid,
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response {
    database: Uuid,
}

pub async fn handler(Json(input): Json<Request>) -> Result<Json<Response>> {
    let database = Kodama::instance()?.create_database(&input.project, &input.name)?;
    Ok(Json(Response { database }))
}
