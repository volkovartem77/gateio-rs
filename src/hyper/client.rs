use crate::http::{Credentials, Method, request::Request};
use crate::hyper::{Error, Response};
use crate::version::VERSION;
use hyper::Uri;
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use http_body_util::Full;
use bytes::Bytes;
use hyper_tls::HttpsConnector;
use std::time::{SystemTime, UNIX_EPOCH};

/// Gate.io async client using hyper.
pub struct GateHttpClient {
    client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    base_url: String,
    credentials: Option<Credentials>,
}

impl GateHttpClient {
    pub fn default() -> Self {
        Self::with_url("https://api.gateio.ws")
    }

    pub fn with_url(url: &str) -> Self {
        use hyper_util::rt::TokioExecutor;
        
        let https = HttpsConnector::new();
        Self {
            client: Client::builder(TokioExecutor::new()).build(https),
            base_url: url.to_string(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub async fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Error> {
        let Request {
            method,
            path,
            params,
            payload,
            x_gate_exp_time,
            credentials,
            sign,
        } = request.into();

        // Map query parameters
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        // Build URL
        let full_url = format!("{}{}?{}", self.base_url, path, query_string);
        let uri: Uri = full_url.parse()?;

        // Create request builder
        let mut req_builder = hyper::Request::builder()
            .uri(uri)
            .header("User-Agent", format!("gateio-rs/{}", VERSION))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        // Handle credentials and signing
        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        
        if let Some(Credentials {
            api_key,
            api_secret,
        }) = request_credentials.or(client_credentials)
        {
            if sign {
                // Use system clock
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Clock may have gone backwards")
                    .as_secs();

                // Set API-Key and Timestamp in header
                req_builder = req_builder
                    .header("KEY", api_key)
                    .header("Timestamp", timestamp.to_string());

                // Set x-gate-exptime header
                if let Some(exp_time_ms) = x_gate_exp_time {
                    req_builder = req_builder.header("x-gate-exptime", exp_time_ms.to_string());
                }

                // Generate signature
                let signature = crate::utils::sign_hmac(
                    method.as_ref(),
                    &path.to_string(),
                    &query_string,
                    &payload,
                    &timestamp.to_string(),
                    api_secret,
                )
                .map_err(|_| Error::InvalidApiSecret)?;

                req_builder = req_builder.header("SIGN", signature);
            }
        }

        // Set method
        req_builder = req_builder.method(match method {
            Method::Get => hyper::Method::GET,
            Method::Post => hyper::Method::POST,
            Method::Put => hyper::Method::PUT,
            Method::Delete => hyper::Method::DELETE,
            Method::Patch => hyper::Method::PATCH,
        });

        // Build request with body
        let body = if payload.is_empty() {
            Full::new(Bytes::new())
        } else {
            Full::new(Bytes::from(payload))
        };

        let request = req_builder
            .body(body)
            .expect("Failed to build request");

        // Send request
        let response = self.client.request(request).await.map_err(|e| Error::Send(Box::new(e)))?;

        Ok(Response::from(response))
    }
}
