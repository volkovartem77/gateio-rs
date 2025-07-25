use crate::http::{Credentials, request::Request};
use crate::ureq::{Error, Response};
use crate::version::VERSION;
use http::Uri;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, Error as UreqError};

/// Synchronous HTTP client for Gate.io API using ureq.
///
/// This client provides blocking I/O operations and is the default client
/// for the Gate.io Rust SDK. It automatically handles request signing,
/// authentication, and provides a simple interface for all API endpoints.
///
/// # Features
///
/// - **Request Signing**: Automatic HMAC SHA-512 signing for authenticated endpoints
/// - **Error Handling**: Comprehensive error handling with detailed error types
/// - **Flexible Configuration**: Configurable base URL, timeouts, and credentials
/// - **Thread Safe**: Can be safely shared across threads using `Arc`
///
/// # Examples
///
/// ## Basic Usage (Public API)
///
/// ```rust,no_run
/// use gateio_rs::{api::spot::get_ticker, ureq::GateHttpClient};
///
/// let client = GateHttpClient::default();
/// let request = get_ticker().currency_pair("BTC_USDT");
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
/// ## Authenticated Usage
///
/// ```rust,no_run
/// use gateio_rs::{
///     api::spot::get_account,
///     http::Credentials,
///     ureq::GateHttpClient,
/// };
///
/// let credentials = Credentials::new("api_key", "api_secret");
/// let client = GateHttpClient::default().credentials(credentials);
/// let request = get_account();
/// let response = client.send(request)?;
/// # Ok::<(), Box<dyn std::error::Error>>(()).expect("");
/// ```
///
/// ## Custom Configuration
///
/// ```rust
/// use gateio_rs::{http::Credentials, ureq::GateHttpClient};
///
/// let client = GateHttpClient::with_url("https://api.gateio.ws")
///     .credentials(Credentials::new("api_key", "api_secret"))
///     .timestamp_delta(1000); // Adjust for server time differences
/// ```
#[derive(Clone)]
pub struct GateHttpClient {
    client: Agent,
    base_url: String,
    timestamp_delta: u64,
    credentials: Option<Credentials>,
}

impl GateHttpClient {
    /// Creates a new client with default settings and Gate.io production URL
    pub fn default() -> Self {
        Self::with_url("https://api.gateio.ws")
    }

    /// Creates a new client with a custom base URL
    pub fn with_url(url: &str) -> Self {
        Self {
            client: Agent::config_builder().build().into(),
            base_url: url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    /// Creates a new client with a custom ureq Agent and base URL
    pub fn with_custom_agent(agent: Agent, url: &str) -> Self {
        Self {
            client: agent,
            base_url: url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    /// Sets the default API credentials for all requests
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Sets the timestamp delta to adjust for server time differences
    pub fn timestamp_delta(mut self, timestamp_delta: u64) -> Self {
        self.timestamp_delta = timestamp_delta;
        self
    }

    /// Sends an HTTP request to the Gate.io API
    pub fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Box<Error>> {
        let Request {
            method,
            path,
            params,
            payload,
            x_gate_exp_time,
            credentials,
            sign,
        } = request.into();

        // Map query parameters (no-ureq)
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        // Build URL
        let full_url: Uri = format!("{}{}?{}", self.base_url, path, query_string).parse()?;

        // Handle different HTTP methods and their respective RequestBuilder types
        let url_string = full_url.to_string();
        let user_agent = &format!("gateio-rs/{}", VERSION);

        // Create common headers
        let headers = vec![
            ("User-Agent", user_agent.as_str()),
            ("Accept", "application/json"),
            ("Content-Type", "application/json"),
        ];

        // Handle credentials and signing
        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        let mut auth_headers: Vec<(&str, String)> = Vec::new();

        if let Some(Credentials {
            api_key,
            api_secret,
        }) = request_credentials.or(client_credentials)
        {
            if sign {
                // Use system clock, panic if system clock is behind `std::time::UNIX_EPOCH`
                let mut timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Clock may have gone backwards")
                    .as_secs();

                // Append timestamp delta to sync up with server time.
                timestamp -= self.timestamp_delta;

                // Set API-Key and Timestamp in header
                auth_headers.push(("KEY", api_key.clone()));
                auth_headers.push(("Timestamp", timestamp.to_string()));

                // Set x-gate-exptime header
                if let Some(exp_time_ms) = x_gate_exp_time {
                    auth_headers.push(("x-gate-exptime", exp_time_ms.to_string()));
                }

                // Stringify available query parameters and append back to query parameters
                let signature = crate::utils::sign_hmac(
                    method.as_ref(),
                    &path.to_string(),
                    &query_string,
                    &payload,
                    &timestamp.to_string(),
                    api_secret,
                )
                .map_err(|_| Error::InvalidApiSecret)?;

                auth_headers.push(("SIGN", signature));
            }
        }

        // Make the request based on method type
        let raw_response = match method {
            crate::http::Method::Get => {
                let mut req = self.client.get(&url_string);
                for (key, value) in &headers {
                    req = req.header(*key, *value);
                }
                for (key, value) in &auth_headers {
                    req = req.header(*key, value.as_str());
                }
                req.call()
            }
            crate::http::Method::Post => {
                let mut req = self.client.post(&url_string);
                for (key, value) in &headers {
                    req = req.header(*key, *value);
                }
                for (key, value) in &auth_headers {
                    req = req.header(*key, value.as_str());
                }
                if payload.is_empty() {
                    req.send_empty()
                } else {
                    req.send(payload.as_bytes())
                }
            }
            crate::http::Method::Put => {
                let mut req = self.client.put(&url_string);
                for (key, value) in &headers {
                    req = req.header(*key, *value);
                }
                for (key, value) in &auth_headers {
                    req = req.header(*key, value.as_str());
                }
                if payload.is_empty() {
                    req.send_empty()
                } else {
                    req.send(payload.as_bytes())
                }
            }
            crate::http::Method::Delete => {
                let mut req = self.client.delete(&url_string);
                for (key, value) in &headers {
                    req = req.header(*key, *value);
                }
                for (key, value) in &auth_headers {
                    req = req.header(*key, value.as_str());
                }
                req.call()
            }
            crate::http::Method::Patch => {
                let mut req = self.client.patch(&url_string);
                for (key, value) in &headers {
                    req = req.header(*key, *value);
                }
                for (key, value) in &auth_headers {
                    req = req.header(*key, value.as_str());
                }
                if payload.is_empty() {
                    req.send_empty()
                } else {
                    req.send(payload.as_bytes())
                }
            }
        };

        let response = match raw_response {
            Ok(response) => Ok(response),
            Err(UreqError::StatusCode(status)) => {
                // In ureq 3.x, StatusCode errors need to be handled differently
                // We need to get the response from the error
                return Err(Box::new(Error::Send(UreqError::StatusCode(status))));
            }
            Err(err) => Err(Error::Send(err)),
        }?;

        Ok(Response::from(response))
    }
}
