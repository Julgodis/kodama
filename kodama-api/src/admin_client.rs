use hyper::{client::HttpConnector, Client, Method};

pub type ClientResult<T> = std::result::Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    InvalidUri(hyper::http::uri::InvalidUri),
    InvalidUriParts(hyper::http::uri::InvalidUriParts),
    SerdeJson(serde_json::Error),
    HyperHttp(hyper::http::Error),
    Hyper(hyper::Error),
    RequestFailed,
    FromUtf8(std::string::FromUtf8Error),
}

impl From<hyper::http::uri::InvalidUri> for ClientError {
    fn from(err: hyper::http::uri::InvalidUri) -> Self {
        Self::InvalidUri(err)
    }
}

impl From<hyper::http::uri::InvalidUriParts> for ClientError {
    fn from(err: hyper::http::uri::InvalidUriParts) -> Self {
        Self::InvalidUriParts(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<hyper::http::Error> for ClientError {
    fn from(err: hyper::http::Error) -> Self {
        Self::HyperHttp(err)
    }
}

impl From<hyper::Error> for ClientError {
    fn from(err: hyper::Error) -> Self {
        Self::Hyper(err)
    }
}

impl From<std::string::FromUtf8Error> for ClientError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(err)
    }
}

pub struct AdminClient {
    uri: hyper::Uri,
    client: Client<HttpConnector>,
}

impl AdminClient {
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
    async fn request<T>(&self, method: hyper::http::Method, path: &str, data: T) -> ClientResult<()>
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
            return Err(ClientError::RequestFailed);
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        tracing::debug!("response: {:?}", body);
        Ok(())
    }

    #[inline]
    async fn request_with_response<T, R>(
        &self,
        method: hyper::http::Method,
        path: &str,
        data: T,
    ) -> ClientResult<R>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
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
            return Err(ClientError::RequestFailed);
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        tracing::debug!("response: {:?}", body);

        let response = serde_json::from_str::<R>(&body)?;
        Ok(response)
    }

    pub async fn create_project(
        self,
        project: impl ToString,
        description: impl ToString,
    ) -> ClientResult<Self> {
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

    pub async fn project_list(self) -> ClientResult<crate::project::ListResponse> {
        self.request_with_response(Method::POST, "/projects", ())
            .await
    }

    pub async fn create_service(
        self,
        project: impl ToString,
        service: impl ToString,
        description: impl ToString,
    ) -> ClientResult<Self> {
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

    pub async fn service_list(
        self,
        project: impl ToString,
    ) -> ClientResult<crate::service::ListResponse> {
        let project = project.to_string();

        self.request_with_response(
            Method::POST,
            "/services",
            crate::service::ListRequest {
                project_name: project.clone(),
            },
        )
        .await
    }

    pub async fn status_sqltrace(
        self,
        project: impl ToString,
        service: impl ToString,
    ) -> ClientResult<crate::sql_trace::StatusResponse> {
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
            return Err(ClientError::RequestFailed);
        }

        let body = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        tracing::debug!("response: {:?}", body);

        let traces = serde_json::from_str::<crate::sql_trace::StatusResponse>(&body)?;
        Ok(traces)
    }

    pub async fn record_list(
        self,
        project: impl ToString,
        service: impl ToString,
    ) -> ClientResult<crate::record::ListResponse> {
        self.request_with_response(
            Method::POST,
            "/records",
            crate::record::ListRequest {
                project_name: project.to_string(),
                service_name: service.to_string(),
            },
        )
        .await
    }

    pub async fn record_entries(
        self,
        project: impl ToString,
        service: impl ToString,
        record: impl ToString,
    ) -> ClientResult<crate::record::DataResponse> {
        self.request_with_response(
            Method::POST,
            "/record",
            crate::record::DataRequest {
                project_name: project.to_string(),
                service_name: service.to_string(),
                record_name: record.to_string(),
            },
        )
        .await
    }
}
