use crate::{
    metric::{Metric, Timestamp},
    Command,
};
use std::net::SocketAddr;

pub struct Client {
    project: String,
    service: String,
    socket_addr: SocketAddr,
}

impl Client {
    pub fn from_socketaddr(
        project: impl ToString,
        service: impl ToString,
        addr: SocketAddr,
    ) -> Self {
        Self {
            project: project.to_string(),
            service: service.to_string(),
            socket_addr: addr,
        }
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            project: self.project.clone(),
            service: self.service.clone(),
            socket_addr: self.socket_addr,
        }
    }
}

impl Client {
    /// Push a metric to the Kodama server. This function is non-blocking.
    #[inline]
    pub fn metric(&self, metric: impl ToString, value: f64) {
        let metric = metric.to_string();
        let timestamp = Timestamp::now();
        self.command(Command::Metric(Metric {
            project_name: self.project.clone(),
            service_name: self.service.clone(),
            metric_name: metric,
            metric_value: value,
            metric_timestamp: timestamp,
        }))
    }

    #[inline]
    pub fn record(&self, record: impl ToString, group_by: impl ToString, execution_time_us: u64) {
        let record: String = record.to_string();
        let group_by = group_by.to_string();
        let timestamp = Timestamp::now();
        self.command(Command::Record(crate::Record {
            project_name: self.project.clone(),
            service_name: self.service.clone(),
            record_name: record,
            group_by,
            timestamp,
            execution_time_us,
            error: 0,
        }))
    }

    #[inline]
    pub fn record_with_error(
        &self,
        record: impl ToString,
        group_by: impl ToString,
        execution_time_us: u64,
        error: bool,
    ) {
        let record = record.to_string();
        let group_by = group_by.to_string();
        let timestamp = Timestamp::now();
        self.command(Command::Record(crate::Record {
            project_name: self.project.clone(),
            service_name: self.service.clone(),
            record_name: record,
            group_by,
            timestamp,
            execution_time_us,
            error: if error { 1 } else { 0 },
        }))
    }

    #[inline]
    fn command(&self, command: Command) {
        let udp_socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("bind");
        let addr = self.socket_addr;
        let data = serde_json::to_vec(&command).expect("serde_json::to_vec");
        udp_socket.send_to(&data, addr).expect("send_to");
    }
}
