use kodama_api::Command;
use kodama_internal::Kodama;
use std::net::SocketAddr;

mod error;

pub type Result<T> = std::result::Result<T, Error>;
pub use error::Error;

fn main() -> Result<()> {
    dotenvy::dotenv().expect("unable to load .env file");

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();

    tracing::debug!("kodama v{}", env!("CARGO_PKG_VERSION"));

    let database_path = std::env::var("KODAMA_DATABASE_PATH").expect("KODAMA_DATABASE_PATH");

    tracing::debug!("- initializing database");
    Kodama::instance(database_path.clone())?.initialize()?;

    let server_database_path = database_path.clone();
    start_data_server(server_database_path)?;

    Ok(())
}

fn handle_request(instance: &mut Kodama, buf: &[u8]) -> Result<()> {
    let utf8_data = std::str::from_utf8(buf)?;
    let data: Command = serde_json::from_str::<Command>(utf8_data)?;

    match data {
        Command::Record(record) => {
            if record.error > 0 {
            } else {
                instance.add_record(
                    &record.project_name,
                    &record.service_name,
                    &record.record_name,
                    &record.group_by,
                    record.timestamp,
                    record.execution_time_us,
                )?;
            }
        }
        _ => {
            tracing::error!("unknown command: {:?}", data);
        }
    }

    Ok(())
}

fn start_data_server(database_path: String) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 49002));
    tracing::debug!("- initializing data server ({})", addr);

    let socket = std::net::UdpSocket::bind(addr)?;
    let mut instance = Kodama::instance(database_path)?;

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf)?;
        tracing::debug!("[{}] {} bytes", addr, len);

        match handle_request(&mut instance, &buf[..len]) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("error: {:?}", err);
            }
        }
    }
}
