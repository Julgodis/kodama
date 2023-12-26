#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateRequest {
    pub project_name: String,
    pub service_name: String,
    pub service_description: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateResponse {
    pub service_id: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListRequest {
    pub project_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub services: Vec<ListService>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListService {
    pub id: i64,
    pub name: String,
    pub description: String,
}
