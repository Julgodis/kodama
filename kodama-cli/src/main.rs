use std::net::SocketAddr;

use clap::Parser;
use kodama_internal::Kodama;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
    #[clap(long)]
    database_path: Option<String>,
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
    let database_path = args
        .database_path
        .unwrap_or_else(|| std::env::var("KODAMA_DATABASE_PATH").expect("KODAMA_DATABASE_PATH"));

    let instance = Kodama::instance(database_path.clone()).expect("kodama instance");

    match args.subcommand {
        SubCommand::Project { subcommand } => project(instance, subcommand).await,
        SubCommand::Service { subcommand } => service(instance, subcommand).await,
        SubCommand::Metric { subcommand } => matric(instance, subcommand).await,
        SubCommand::Record { subcommand } => record(instance, subcommand).await,
    }
}

async fn project(kodama: Kodama, subcommand: ProjectSubCommand) {
    match subcommand {
        ProjectSubCommand::Create { name, description } => {
            tracing::debug!("creating project: {:?}", name);
            kodama
                .create_project(&name, &description)
                .expect("create project");
        }
        ProjectSubCommand::List => {
            tracing::debug!("listing projects");
            let projects = kodama.project_list().expect("project list");

            println!();
            println!("projects:");
            println!("{: >10} {: <80}", "[id]", "[name]");
            for project in &projects {
                println!("{: >10} {: <80}", project.id, project.name);
            }
        }
    }
}

async fn service(kodama: Kodama, subcommand: ServiceSubCommand) {
    match subcommand {
        ServiceSubCommand::Create {
            project,
            name,
            description,
        } => {
            tracing::debug!("creating service: {:?}", name);
            kodama
                .create_service(&project, &name, &description)
                .expect("create service");
        }
        ServiceSubCommand::List { project } => {
            tracing::debug!("listing services");

            let services = kodama.service_list(&project).expect("service list");
            println!();
            println!("services:");
            println!("{: >10} {: <80}", "[id]", "[name]");
            for service in &services {
                println!("{: >10} {: <80}", service.id, service.name);
            }
        }
    }
}

async fn matric(_kodama: Kodama, subcommand: MetricSubCommand) {
    match subcommand {
        MetricSubCommand::Push {
            project,
            service,
            metric,
            value,
        } => {
            tracing::debug!("pushing metric: {:?}", metric);

            let instance = kodama_api::Client::from_socketaddr(
                project,
                service,
                SocketAddr::from(([127, 0, 0, 1], 49001)),
            );

            instance.metric(&metric, value);
        }
    }
}

async fn record(mut kodama: Kodama, subcommand: RecordSubCommand) {
    match subcommand {
        RecordSubCommand::List { project, service } => {
            let records = kodama.record_list(&project, &service).expect("record list");

            println!();
            println!("{: >10} {: <80}", "[id]", "[name]");
            for record in &records {
                println!("{: >10} {: <80}", record.id, record.name);
            }
        }
        RecordSubCommand::Data {
            project,
            service,
            record,
        } => {
            let mut queries = kodama
                .record_entries(&project, &service, &record)
                .expect("record entries");

            println!();
            println!(
                "{: >10} {: >10} {: >10} {: >10} {: >10} {: <80}",
                "[total]", "[avg]", "[p50]", "[p95]", "[count]", "[query]"
            );
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
