use crate::error::ApiError;
use crate::kodama::Kodama;
use crate::Result;
use axum::{Json, extract::State};
use kodama_api::service::{CreateRequest, CreateResponse};

pub async fn handler(
    State(database_path): State<String>,
    Json(input): Json<CreateRequest>,
) -> Result<Json<CreateResponse>> {
    // service names must be in kebab-case
    if !input
        .service_name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-')
    {
        return Err(ApiError::InvalidServiceName.into());
    }

    tracing::info!("create service: {:?}", input);
    let service_id = Kodama::instance(database_path)?.create_service(
        &input.project_name,
        &input.service_name,
        &input.service_description,
    )?;
    Ok(Json(CreateResponse { service_id }))
}
