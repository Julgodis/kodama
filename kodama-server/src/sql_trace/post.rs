use crate::kodama::Kodama;
use crate::Result;
use axum::Json;
use kodama_api::sql_trace::{StatusRequest, StatusResponse};

pub async fn handler(Json(input): Json<StatusRequest>) -> Result<Json<StatusResponse>> {
    tracing::info!("status sql-trace: {:?}", input);
    let queries = Kodama::instance()?.status_sqltrace(&input.project_name, &input.service_name)?;
    Ok(Json(StatusResponse { queries }))
}
