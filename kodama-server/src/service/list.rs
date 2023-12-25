use crate::kodama::Kodama;
use crate::Result;
use axum::extract::State;
use axum::Json;
use kodama_api::service::ListRequest;
use kodama_api::service::ListResponse;

pub async fn handler(
    State(database_path): State<String>,
    Json(input): Json<ListRequest>,
) -> Result<Json<ListResponse>> {
    tracing::info!("list services");
    let services = Kodama::instance(database_path)?.service_list(&input.project_name)?;
    Ok(Json(ListResponse { services }))
}
