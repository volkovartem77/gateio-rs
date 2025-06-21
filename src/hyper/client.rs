use crate::http::{request::Request, Credentials, Method};
use crate::hyper::{Error, Response};
use crate::version::VERSION;
use hyper::{client::connect::Connect, client::HttpConnector, Body, Client, Uri};
use hyper_tls::HttpsConnector;
use std::time::{SystemTime, UNIX_EPOCH};

/// Gate.io async client using hyper.
pub struct GateHttpClient {
    client: Client<HttpsConnector<hyper::client::HttpConnector>, Body>,
    base_url: String,
    credentials: Option<Credentials>,
}

impl GateHttpClient {
    pub fn default() -> Self {
        Self::with_url("https://api.gateio.ws")
    }

    pub fn with_url(url: &str) -> Self {
        Self {
            client: Client::builder().build(HttpsConnector::new()),
            base_url: url.to_string(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub async fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Error> {
        // логика формирования URL, подписи, заголовков и выполнения запроса
        todo!()
    }
}