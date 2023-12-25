use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "project")]
    Project {
        #[clap(subcommand)]
        subcommand: ProjectSubCommand,
    },

    #[clap(name = "service")]
    Service {
        #[clap(subcommand)]
        subcommand: ServiceSubCommand,
    },

    #[clap(name = "metric")]
    Metric {
        #[clap(subcommand)]
        subcommand: MetricSubCommand,
    },

    #[clap(name = "sqltrace")]
    SqlTrace {
        #[clap(subcommand)]
        subcommand: SqlTraceSubCommand,
    },

    #[clap(name = "record")]
    Record {
        #[clap(subcommand)]
        subcommand: RecordSubCommand,
    },
}

#[derive(Parser)]
enum ProjectSubCommand {
    #[clap(name = "create")]
    Create { name: String, description: String },
    #[clap(name = "list", alias = "ls")]
    List,
}

#[derive(Parser)]
enum ServiceSubCommand {
    #[clap(name = "create")]
    Create {
        project: String,
        name: String,
        description: String,
    },
    #[clap(name = "list", alias = "ls")]
    List { project: String },
}

#[derive(Parser)]
enum MetricSubCommand {
    #[clap(name = "push")]
    Push {
        project: String,
        service: String,
        metric: String,
        value: f64,
    },
}

#[derive(Parser)]
enum RecordSubCommand {
    #[clap(name = "list", alias = "ls")]
    List { project: String, service: String },
    #[clap(name = "data")]
    Data {
        project: String,
        service: String,
        record: String,
    },
}

/*
CREATE TABLE IF NOT EXISTS sql_traces (
    trace_id INTEGER PRIMARY KEY,
    service_id INTEGER NOT NULL,
    -- 0 = SELECT
    -- 1 = INSERT
    -- 2 = UPDATE
    -- 3 = DELETE
    -- 4 = CREATE
    -- 5 = ALTER
    -- 6 = DROP
    -- 7 = TRUNCATE
    -- 8 = COMMENT
    -- 9 = SET
    command_type INTEGER NOT NULL,
    query TEXT NOT NULL,
    expended_query TEXT NOT NULL,
    -- microseconds
    execution_time INTEGER NOT NULL,
    row_changes INTEGER NOT NULL,
    last_row_id INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES services(service_id)
);
*/

#[derive(Parser)]
enum SqlTraceSubCommand {
    /// List all sql traces (grouped by query) with p50, p95, and p99 execution times
    /// in order of p99 execution time.
    #[clap(name = "list")]
    List { project: String, service: String },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();

    tracing::debug!("kodama-cli v{}", env!("CARGO_PKG_VERSION"));

    let args = Cli::parse();
    match args.subcommand {
        SubCommand::Project { subcommand } => project(subcommand).await,
        SubCommand::Service { subcommand } => service(subcommand).await,
        SubCommand::Metric { subcommand } => matric(subcommand).await,
        SubCommand::SqlTrace { subcommand } => sqltrace(subcommand).await,
        SubCommand::Record { subcommand } => record(subcommand).await,
    }
}

async fn project(subcommand: ProjectSubCommand) {
    match subcommand {
        ProjectSubCommand::Create { name, description } => {
            tracing::debug!("creating project: {:?}", name);

            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .create_project(&name, &description)
                .await
            {
                Ok(_) => {
                    tracing::debug!("  project created");
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
        ProjectSubCommand::List => {
            tracing::debug!("listing projects");

            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri).project_list().await {
                Ok(response) => {
                    println!("projects:");
                    println!("{: >10} {: <80}", "[id]", "[name]");
                    for project in &response.projects {
                        println!("{: >10} {: <80}", project.id, project.name);
                    }
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
    }
}

async fn service(subcommand: ServiceSubCommand) {
    match subcommand {
        ServiceSubCommand::Create {
            project,
            name,
            description,
        } => {
            tracing::debug!("creating service: {:?}", name);

            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .create_service(&project, &name, &description)
                .await
            {
                Ok(_) => {
                    tracing::debug!("  service created");
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
        ServiceSubCommand::List { project } => {
            tracing::debug!("listing services");

            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .service_list(&project)
                .await
            {
                Ok(response) => {
                    println!("services:");
                    println!("{: >10} {: <80}", "[id]", "[name]");
                    for service in &response.services {
                        println!("{: >10} {: <80}", service.id, service.name);
                    }
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
    }
}

async fn matric(subcommand: MetricSubCommand) {
    match subcommand {
        MetricSubCommand::Push {
            project,
            service,
            metric,
            value,
        } => {
            tracing::debug!("pushing metric: {:?}", metric);

            let instance = kodama_api::Client::from_socketaddr(
                &project,
                &service,
                SocketAddr::from(([127, 0, 0, 1], 49001)),
            );

            instance.metric(&metric, value);
        }
    }
}

async fn sqltrace(subcommand: SqlTraceSubCommand) {
    match subcommand {
        SqlTraceSubCommand::List { project, service } => {
            tracing::debug!("listing sql traces");

            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .status_sqltrace(&project, &service)
                .await
            {
                Ok(response) => {
                    println!("");
                    println!(
                        "{: >10} {: >10} {: >10} {: >10} {: >10} {: <80}",
                        "[avg]", "[p50]", "[p95]", "[p99]", "[count]", "[query]"
                    );
                    let mut queries = response.queries;
                    queries.sort_by(|a, b| b.p99.cmp(&a.p99));
                    for trace in &queries {
                        let query = if trace.query.len() > 80 - 3 {
                            format!("{}...", &trace.query[..(80 - 3)])
                        } else {
                            trace.query.clone()
                        };
                        println!(
                            "{: >10} {: >10} {: >10} {: >10} {: >10} {: <80}",
                            trace.avg, trace.p50, trace.p95, trace.p99, trace.count, query
                        );
                    }
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
    }
}

async fn record(subcommand: RecordSubCommand) {
    match subcommand {
        RecordSubCommand::List { project, service } => {
            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .record_list(&project, &service)
                .await
            {
                Ok(response) => {
                    println!("");
                    println!("{: >10} {: <80}", "[id]", "[name]");
                    for record in &response.records {
                        println!("{: >10} {: <80}", record.id, record.name);
                    }
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
        RecordSubCommand::Data {
            project,
            service,
            record,
        } => {
            let uri = "http://localhost:49001".parse().expect("uri");
            match kodama_api::AdminClient::from_uri(uri)
                .record_entries(&project, &service, &record)
                .await
            {
                Ok(response) => {
                    println!("");
                    println!(
                        "{: >10} {: >10} {: >10} {: >10} {: >10} {: <80}",
                        "[total]", "[avg]", "[p50]", "[p95]", "[count]", "[query]"
                    );
                    let mut queries = response.entries;
                    queries.sort_by(|a, b| b.p95.cmp(&a.p95));
                    for trace in &queries {
                        let query = if trace.group_by.len() > 80 - 3 {
                            format!("{}...", &trace.group_by[..(80 - 3)])
                        } else {
                            trace.group_by.clone()
                        };
                        println!(
                            "{: >10} {: >10} {: >10} {: >10} {: >10} {: <80}",
                            us_to_human(trace.execution_time),
                            us_to_human(trace.avg),
                            us_to_human(trace.p50),
                            us_to_human(trace.p95),
                            trace.count,
                            query
                        );
                    }
                }
                Err(err) => {
                    tracing::error!("error: {:?}", err);
                }
            }
        }
    }
}

fn us_to_human(us: u64) -> String {
    if us < 1000 {
        format!("{}us", us)
    } else if us < 1000 * 1000 {
        format!("{:.2}ms", us as f64 / 1000.0)
    } else if us < 1000 * 1000 * 1000 {
        format!("{:.2}s", us as f64 / 1000.0 / 1000.0)
    } else if us < 1000 * 1000 * 1000 * 60 * 60 {
        format!("{:.2}m", us as f64 / 1000.0 / 1000.0 / 60.0)
    } else if us < 1000 * 1000 * 1000 * 60 * 60 * 24 {
        format!("{:.2}h", us as f64 / 1000.0 / 1000.0 / 60.0 / 60.0)
    } else {
        format!("{:.2}d", us as f64 / 1000.0 / 1000.0 / 60.0 / 60.0 / 24.0)
    }
}
