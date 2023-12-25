use crate::error::ApiError;
use crate::kodama::Kodama;
use crate::Result;
use axum::Json;
use kodama_api::sql_trace::{PushRequest, PushResponse};

pub async fn handler(Json(input): Json<PushRequest>) -> Result<Json<PushResponse>> {
    tracing::info!("create sql-trace: {:?}", input);

    let timestamp = match input.timestamp {
        Some(x) => x
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|_| ApiError::InvalidTimestamp)?,
        None => chrono::Utc::now(),
    };

    let trace_id = Kodama::instance()?.create_sqltrace(
        &input.project_name,
        &input.service_name,
        input.command_type,
        &input.query,
        input.expanded_query.as_ref(),
        input.execution_time,
        input.row_changes,
        input.last_row_id,
        timestamp,
    )?;

    Ok(Json(PushResponse { trace_id }))
}
