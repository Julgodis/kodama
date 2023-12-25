#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Command {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Alter,
    Drop,
    Truncate,
    Comment,
    Set,
}

impl Command {
    pub fn to_i64(&self) -> i64 {
        match self {
            Command::Select => 1,
            Command::Insert => 2,
            Command::Update => 3,
            Command::Delete => 4,
            Command::Create => 5,
            Command::Alter => 6,
            Command::Drop => 7,
            Command::Truncate => 8,
            Command::Comment => 9,
            Command::Set => 10,
        }
    }

    pub fn from_i64(i: i64) -> Option<Self> {
        match i {
            1 => Some(Command::Select),
            2 => Some(Command::Insert),
            3 => Some(Command::Update),
            4 => Some(Command::Delete),
            5 => Some(Command::Create),
            6 => Some(Command::Alter),
            7 => Some(Command::Drop),
            8 => Some(Command::Truncate),
            9 => Some(Command::Comment),
            10 => Some(Command::Set),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushRequest {
    pub project_name: String,
    pub service_name: String,
    pub command_type: Command,
    pub query: String,
    pub expanded_query: Option<String>,
    pub execution_time: u64,
    pub row_changes: i64,
    pub last_row_id: i64,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushResponse {
    pub trace_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusRequest {
    pub project_name: String,
    pub service_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    pub queries: Vec<StatusQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusQuery {
    /// Query string
    pub query: String,
    /// Total execution count
    pub count: i64,
    /// Total execution time in microseconds
    pub execution_time: u64,
    /// Minimum execution time in microseconds
    pub min: u64,
    /// Maximum execution time in microseconds
    pub max: u64,
    /// Average execution time in microseconds
    pub avg: u64,
    /// Execution time 50th percentile in microseconds
    pub p50: u64,
    /// Execution time 95th percentile in microseconds
    pub p95: u64,
    /// Execution time 99th percentile in microseconds
    pub p99: u64,
}
