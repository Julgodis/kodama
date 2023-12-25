use std::net::SocketAddr;

use hyper::{client::HttpConnector, Client, Method};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::sql_trace;

pub struct Kodama {
    uri: hyper::Uri,
    client: Client<HttpConnector>,
}

impl Kodama {
    pub fn from_uri(uri: hyper::Uri) -> Self {
        let client = Client::builder()
            .http2_only(false)
            .http2_initial_stream_window_size(1024 * 1024)
            .build_http();

        Self { client, uri }
    }

    pub fn from_socketaddr(addr: std::net::SocketAddr) -> Self {
        let uri = format!("http://{}", addr).parse().expect("uri");
        Self::from_uri(uri)
    }

    #[inline]
    async fn request<T>(
        &self,
        method: hyper::http::Method,
        path: &str,
        data: T,
    ) -> InstanceResult<()>
    where
        T: serde::Serialize,
    {
        let data = serde_json::to_string(&data)?;

        let mut parts = self.uri.clone().into_parts();
        parts.path_and_query = Some(path.parse()?);
        let uri = hyper::Uri::from_parts(parts)?;

        tracing::debug!("request: {:?} {:?}", uri, data);
        let req = hyper::Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(hyper::Body::from(data))?;

        let resp = self.client.request(req).await?;
        if resp.status() != hyper::StatusCode::OK {
            return Err(InstanceError::RequestFailed);
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        tracing::debug!("response: {:?}", body);
        Ok(())
    }

    pub async fn create_project(
        self,
        project: impl ToString,
        description: impl ToString,
    ) -> InstanceResult<Self> {
        let project = project.to_string();
        let description = description.to_string();

        self.request(
            Method::PUT,
            "/project",
            crate::project::CreateRequest {
                project_name: project.clone(),
                project_description: description.clone(),
            },
        )
        .await?;
        Ok(self)
    }

    pub async fn create_service(
        self,
        project: impl ToString,
        service: impl ToString,
        description: impl ToString,
    ) -> InstanceResult<Self> {
        let project = project.to_string();
        let service = service.to_string();
        let description = description.to_string();

        self.request(
            Method::PUT,
            "/service",
            crate::service::CreateRequest {
                project_name: project.clone(),
                service_name: service.clone(),
                service_description: description.clone(),
            },
        )
        .await?;
        Ok(self)
    }

    pub async fn status_sqltrace(
        self,
        project: impl ToString,
        service: impl ToString,
    ) -> InstanceResult<crate::sql_trace::StatusResponse> {
        let project = project.to_string();
        let service = service.to_string();

        let data = crate::sql_trace::StatusRequest {
            project_name: project.clone(),
            service_name: service.clone(),
        };

        let data = serde_json::to_string(&data)?;

        let mut parts = self.uri.clone().into_parts();
        parts.path_and_query = Some("/sql-trace".parse()?);
        let uri = hyper::Uri::from_parts(parts)?;

        tracing::debug!("request: {:?}", uri);
        let req = hyper::Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(hyper::Body::from(data))?;

        let resp = self.client.request(req).await?;
        if resp.status() != hyper::StatusCode::OK {
            return Err(InstanceError::RequestFailed);
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        tracing::debug!("response: {:?}", body);

        let traces = serde_json::from_str::<crate::sql_trace::StatusResponse>(&body)?;
        Ok(traces)
    }

    pub async fn raw_metric(self, data: crate::metric::PushRequest) -> InstanceResult<Self> {
        tracing::debug!("metric: {:?}", data);
        self.request(Method::PUT, "/metric", data).await?;
        Ok(self)
    }

    pub async fn metric_with_timestamp(
        self,
        project: impl ToString,
        service: impl ToString,
        metric: impl ToString,
        value: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> InstanceResult<Self> {
        self.raw_metric(crate::metric::PushRequest {
            project_name: project.to_string(),
            service_name: service.to_string(),
            metric_name: metric.to_string(),
            metric_value: value,
            metric_timestamp: Some(timestamp.to_rfc3339()),
        })
        .await
    }

    pub async fn metric(
        self,
        project: impl ToString,
        service: impl ToString,
        metric: impl ToString,
        value: f64,
    ) -> InstanceResult<Self> {
        self.raw_metric(crate::metric::PushRequest {
            project_name: project.to_string(),
            service_name: service.to_string(),
            metric_name: metric.to_string(),
            metric_value: value,
            metric_timestamp: None,
        })
        .await
    }

    pub async fn sqltrace(
        self,
        project: impl ToString,
        service: impl ToString,
        command_type: sql_trace::Command,
        query: impl ToString,
        expanded_query: Option<String>,
        execution_time: u64,
        row_changes: i64,
        last_row_id: i64,
    ) -> InstanceResult<Self> {
        let query = query.to_string();

        self.request(
            Method::PUT,
            "/sql-trace",
            crate::sql_trace::PushRequest {
                project_name: project.to_string(),
                service_name: service.to_string(),
                command_type,
                query,
                expanded_query,
                execution_time,
                row_changes,
                last_row_id,
                timestamp: None,
            },
        )
        .await?;

        Ok(self)
    }
}
