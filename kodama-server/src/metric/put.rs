use crate::error::ApiError;
use crate::kodama::Kodama;
use crate::Result;
use axum::Json;
use kodama_api::metric::{PushRequest, PushResponse};

pub async fn handler(Json(input): Json<PushRequest>) -> Result<Json<PushResponse>> {
    tracing::info!("create metric: {:?}", input);

    let timestamp = match input.metric_timestamp {
        Some(x) => x
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|_| ApiError::InvalidTimestamp)?,
        None => chrono::Utc::now(),
    };

    let metric_id = Kodama::instance()?.create_metric(
        &input.project_name,
        &input.service_name,
        &input.metric_name,
        input.metric_value,
        timestamp,
    )?;

    Ok(Json(PushResponse { metric_id }))
}
