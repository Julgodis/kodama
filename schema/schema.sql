CREATE TABLE IF NOT EXISTS projects (
    project_id INTEGER PRIMARY KEY,
    project_name TEXT NOT NULL,
    description TEXT,
    UNIQUE (project_name)
);

CREATE TABLE IF NOT EXISTS services (
    service_id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    service_name TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(project_id),
    UNIQUE (project_id, service_name)
);

CREATE TABLE IF NOT EXISTS metrics (
    metric_id INTEGER PRIMARY KEY,
    service_id INTEGER NOT NULL,
    metric_name TEXT NOT NULL,
    FOREIGN KEY (service_id) REFERENCES services(service_id)
);

CREATE TABLE IF NOT EXISTS records (
    record_id INTEGER PRIMARY KEY,
    service_id INTEGER NOT NULL,
    record_name TEXT NOT NULL,
    FOREIGN KEY (service_id) REFERENCES services(service_id)
);

CREATE INDEX IF NOT EXISTS idx_records_service_id_record_name ON records (service_id, record_name);