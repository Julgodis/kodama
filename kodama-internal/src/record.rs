#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListRequest {
    pub project_name: String,
    pub service_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListResponse {
    pub records: Vec<ListRecord>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListRecord {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataRequest {
    pub project_name: String,
    pub service_name: String,
    pub record_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataResponse {
    pub entries: Vec<DataEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataEntry {
    /// Group by value
    pub group_by: String,
    /// Total record count
    pub count: i64,
    /// Total record errors
    pub errors: i64,
    /// Total execution time in microseconds
    pub execution_time: u64,
    /// Minimum record execution time in microseconds
    pub min: u64,
    /// Maximum record execution time in microseconds
    pub max: u64,
    /// Average record execution time in microseconds
    pub avg: u64,
    /// Execution time 50th percentile in microseconds
    pub p50: u64,
    /// Execution time 95th percentile in microseconds
    pub p95: u64,
}
