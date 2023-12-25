use crate::kodama::Kodama;
use crate::Result;
use axum::Json;
use axum::extract::State;
use kodama_api::project::ListResponse;

pub async fn handler(State(database_path): State<String>) -> Result<Json<ListResponse>> {
    tracing::info!("list project");
    let projects = Kodama::instance(database_path)?.project_list()?;
    Ok(Json(ListResponse { projects }))
}
