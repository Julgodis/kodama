use kodama_api::Timestamp;
use project::ListProject;
use record::{DataEntry, ListRecord};
use service::ListService;
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

mod error;
pub use error::*;
pub mod metric;
pub mod project;
pub mod record;
pub mod service;

struct Service {
    id: i64,
    db: rusqlite::Connection,
}

impl Service {
    pub fn open(path: &String, service_id: i64) -> Result<Self> {
        let path = PathBuf::from(path).join(format!("service-{}.db", service_id));
        let db = rusqlite::Connection::open(path)?;
        // enable foreign key constraints
        db.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Self { id: service_id, db })
    }

    /// Create a table to store record data
    pub fn define_record(&self, record_id: i64) -> Result<()> {
        tracing::debug!("define record {}", record_id);
        self.db.execute_batch(&format!(
            "CREATE TABLE IF NOT EXISTS record_{} (
            timestamp INTEGER PRIMARY KEY,
            group_by TEXT NOT NULL,
            execution_time_us INTEGER NOT NULL,
            error INTEGER DEFAULT 0
        );",
            record_id
        ))?;

        Ok(())
    }

    pub fn add_record(
        &self,
        record_id: i64,
        timestamp: Option<Timestamp>,
        group_by: &str,
        execution_time: u64,
    ) -> Result<()> {
        let mut stmt = self.db.prepare(&format!(
            "INSERT INTO record_{} (timestamp, group_by, execution_time_us) VALUES (?1, ?2, ?3)",
            record_id
        ))?;

        let timestamp = if let Some(timestamp) = timestamp {
            timestamp
        } else if let Some(timestamp) = Timestamp::now() {
            timestamp
        } else {
            return Err(ApiError::InvalidTimestamp.into());
        };

        stmt.execute(rusqlite::params![timestamp, group_by, execution_time])?;
        Ok(())
    }

    pub fn record_entries(&self, record_id: i64) -> Result<Vec<DataEntry>> {
        let mut stmt = self.db.prepare(&format!(
            "
SELECT 
group_by, 
COUNT(*), 
SUM(execution_time_us), 
AVG(execution_time_us), 
MAX(execution_time_us), 
MIN(execution_time_us),
COUNT(CASE WHEN error > 0 THEN 1 ELSE NULL END) AS error_count
FROM record_{}
GROUP BY group_by",
            record_id
        ))?;
        let mut rows = stmt
            .query_map(rusqlite::params![], |row| {
                let avg: f64 = row.get(3)?;
                let avg_rounded = avg.round() as u64;
                Ok(DataEntry {
                    group_by: row.get(0)?,
                    count: row.get(1)?,
                    errors: row.get(6)?,
                    execution_time: row.get(2)?,
                    avg: avg_rounded,
                    max: row.get(4)?,
                    min: row.get(5)?,
                    p50: 0,
                    p95: 0,
                })
            })?
            .inspect(|x| {
                if let Err(e) = x {
                    tracing::error!("error: {:?}", e);
                }
            })
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();

        for row in rows.iter_mut() {
            let percentile_50 = row.count * 50 / 100;
            let percentile_95 = row.count * 95 / 100;

            let p50 = self
            .db
            .prepare(&format!("SELECT execution_time_us FROM record_{} WHERE group_by = ?1 ORDER BY execution_time_us ASC LIMIT 1 OFFSET ?2", record_id))?
            .query_row(rusqlite::params![row.group_by, percentile_50], |row| row.get(0))?;
            let p95 = self
            .db
            .prepare(&format!("SELECT execution_time_us FROM record_{} WHERE group_by = ?1 ORDER BY execution_time_us ASC LIMIT 1 OFFSET ?2", record_id))?
            .query_row(rusqlite::params![row.group_by, percentile_95], |row| row.get(0))?;

            row.p50 = p50;
            row.p95 = p95;
        }

        Ok(rows)
    }
}

type ServiceRef = Rc<RefCell<Service>>;

pub struct Kodama {
    db: rusqlite::Connection,
    database_path: String,
    services_by_ps: HashMap<(String, String), ServiceRef>,
    services_by_id: HashMap<i64, ServiceRef>,
}

impl Kodama {
    pub fn instance(path: String) -> Result<Self> {
        let database_path = path.clone();
        std::fs::create_dir_all(&path).map_err(|_| ApiError::UnableToCreateDatabasePath)?;

        let path = PathBuf::from(path).join("kodama.db");
        let db = rusqlite::Connection::open(path)?;
        // enable foreign key constraints
        db.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Self {
            db,
            database_path,
            services_by_ps: HashMap::new(),
            services_by_id: HashMap::new(),
        })
    }

    pub fn initialize(self) -> Result<Self> {
        self.db
            .execute_batch(include_str!("../../schema/schema.sql"))?;
        Ok(self)
    }

    pub fn create_project(&self, project_name: &str, description: &str) -> Result<i64> {
        let mut stmt = self
            .db
            .prepare("INSERT INTO projects (project_name, description) VALUES (?1, ?2)")?;
        let project_id = stmt.insert(rusqlite::params![project_name, description])?;
        Ok(project_id)
    }

    pub fn project_list(&self) -> Result<Vec<ListProject>> {
        let mut stmt = self
            .db
            .prepare("SELECT project_id, project_name, description FROM projects")?;
        let rows = stmt.query_map(rusqlite::params![], |row| {
            Ok(ListProject {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }
        Ok(projects)
    }

    pub fn get_project_id(&self, project_name: &str) -> Result<i64> {
        let mut stmt = self
            .db
            .prepare("SELECT project_id FROM projects WHERE project_name = ?1")?;
        let mut rows = stmt.query(rusqlite::params![project_name])?;
        let row = rows.next()?.ok_or(ApiError::ProjectNotFound)?;
        let project_id = row.get(0)?;
        Ok(project_id)
    }

    pub fn create_service(
        &self,
        project_name: &str,
        service_name: &str,
        description: &str,
    ) -> Result<i64> {
        let project_id = self.get_project_id(project_name)?;
        let mut stmt = self.db.prepare(
            "INSERT INTO services (project_id, service_name, description) VALUES (?1, ?2, ?3)",
        )?;
        let service_id = stmt.insert(rusqlite::params![project_id, service_name, description])?;
        Ok(service_id)
    }

    pub fn service_list(&self, project_name: &str) -> Result<Vec<ListService>> {
        let project_id = self.get_project_id(project_name)?;
        let mut stmt = self.db.prepare(
            "SELECT service_id, service_name, description FROM services WHERE project_id = ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![project_id], |row| {
            Ok(ListService {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        let mut services = Vec::new();
        for row in rows {
            services.push(row?);
        }
        Ok(services)
    }

    pub fn get_service_id(&self, project_name: &str, service_name: &str) -> Result<i64> {
        let mut stmt = self.db.prepare(
            "    SELECT s.service_id
            FROM services AS s
            JOIN projects AS p ON s.project_id = p.project_id
            WHERE p.project_name = ?1 AND s.service_name = ?2",
        )?;
        let mut rows = stmt.query(rusqlite::params![project_name, service_name])?;
        let row = rows
            .next()?
            .ok_or(ApiError::ServiceNotFound(service_name.to_string()))?;
        let service_id = row.get(0)?;
        Ok(service_id)
    }

    fn get_service(&mut self, project_name: &str, service_name: &str) -> Result<ServiceRef> {
        let key = (project_name.to_string(), service_name.to_string());
        if !self.services_by_ps.contains_key(&key) {
            let service_id = self.get_service_id(project_name, service_name)?;
            let service = Service::open(&self.database_path, service_id)?;
            let service = Rc::new(RefCell::new(service));
            self.services_by_ps.insert(key.clone(), service.clone());
            self.services_by_id.insert(service_id, service.clone());
            Ok(service)
        } else {
            let service = self.services_by_ps.get(&key).unwrap();
            Ok(service.clone())
        }
    }

    fn create_or_get_record_table(
        &mut self,
        project_name: &str,
        service_name: &str,
        record_name: &str,
    ) -> Result<(ServiceRef, i64)> {
        let service = self.get_service(project_name, service_name)?;
        let service_id = service.borrow().id;

        let mut stmt = self
            .db
            .prepare("SELECT record_id FROM records WHERE service_id = ?1 AND record_name = ?2")?;
        let mut rows = stmt.query(rusqlite::params![service_id, record_name])?;
        let row = rows.next()?;
        if let Some(row) = row {
            let record_id = row.get(0)?;
            Ok((service, record_id))
        } else {
            let mut stmt = self
                .db
                .prepare("INSERT INTO records (service_id, record_name) VALUES (?1, ?2)")?;
            let record_id = stmt.insert(rusqlite::params![service_id, record_name])?;
            service.borrow_mut().define_record(record_id)?;
            Ok((service, record_id))
        }
    }

    pub fn add_record(
        &mut self,
        project_name: &str,
        service_name: &str,
        record_name: &str,
        group_by: &str,
        timestamp: Option<Timestamp>,
        execution_time: u64,
    ) -> Result<()> {
        let (service, record_id) =
            self.create_or_get_record_table(project_name, service_name, record_name)?;

        service
            .borrow()
            .add_record(record_id, timestamp, group_by, execution_time)?;
        Ok(())
    }

    pub fn record_list(&self, project_name: &str, service_name: &str) -> Result<Vec<ListRecord>> {
        let service_id = self.get_service_id(project_name, service_name)?;
        let mut stmt = self
            .db
            .prepare("SELECT record_id, record_name FROM records WHERE service_id = ?1")?;
        let records = stmt
            .query_map(rusqlite::params![service_id], |row| {
                Ok(ListRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();
        Ok(records)
    }

    fn get_record_id(&self, service_id: i64, record_name: &str) -> Result<i64> {
        let mut stmt = self
            .db
            .prepare("SELECT record_id FROM records WHERE service_id = ?1 AND record_name = ?2")?;
        let mut rows = stmt.query(rusqlite::params![service_id, record_name])?;
        let row = rows.next()?.ok_or(ApiError::RecordNotFound)?;
        let record_id = row.get(0)?;
        Ok(record_id)
    }

    pub fn record_entries(
        &mut self,
        project_name: &str,
        service_name: &str,
        record_name: &str,
    ) -> Result<Vec<DataEntry>> {
        let service = self.get_service(project_name, service_name)?;
        let record_id = self.get_record_id(service.borrow().id, record_name)?;
        let entries = service.borrow().record_entries(record_id)?;
        Ok(entries)
    }
}
