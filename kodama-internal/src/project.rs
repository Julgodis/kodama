#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateRequest {
    pub project_name: String,
    pub project_description: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateResponse {
    pub project_id: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub projects: Vec<ListProject>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListProject {
    pub id: i64,
    pub name: String,
    pub description: String,
}
