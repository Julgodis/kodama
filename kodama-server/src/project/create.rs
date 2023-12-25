use crate::error::ApiError;
use crate::kodama::Kodama;
use crate::Result;
use axum::{Json, extract::State};
use kodama_api::project::{CreateRequest, CreateResponse};

pub async fn handler(
    State(database_path): State<String>,
    Json(input): Json<CreateRequest>,
) -> Result<Json<CreateResponse>> {
    // project names must be in kebab-case
    if !input
        .project_name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-')
    {
        return Err(ApiError::InvalidProjectName.into());
    }

    tracing::info!("create project: {:?}", input);
    let project_id = Kodama::instance(database_path)?
        .create_project(&input.project_name, &input.project_description)?;
    Ok(Json(CreateResponse { project_id }))
}
