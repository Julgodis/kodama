#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushRequest {
    pub project_name: String,
    pub service_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub metric_timestamp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushResponse {
    pub metric_id: i64,
}
