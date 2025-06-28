use crate::http::{request::Request, Credentials};
use crate::ureq::{Error, Response};
use crate::version::VERSION;
use http::Uri;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, AgentBuilder, Error as UreqError};

/// #### Gate.io sync client using ureq
#[derive(Clone)]
pub struct GateHttpClient {
    client: Agent,
    base_url: String,
    timestamp_delta: u64,
    credentials: Option<Credentials>,
}

impl GateHttpClient {
    pub fn default() -> Self {
        Self::with_url("https://api.gateio.ws")
    }

    pub fn with_url(url: &str) -> Self {
        Self {
            client: AgentBuilder::new().build(),
            base_url: url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn timestamp_delta(mut self, timestamp_delta: u64) -> Self {
        self.timestamp_delta = timestamp_delta;
        self
    }

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

        let mut ureq_request = self.client.request(method.as_ref(), &full_url.to_string());

        // Set User-Agent in header
        let user_agent = &format!("gateio-rs/{}", VERSION);
        ureq_request = ureq_request
            .set("User-Agent", user_agent)
            .set("Accept", "application/json")
            .set("Content-Type", "application/json");

        // Insert credentials
        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        if let Some(Credentials { api_key, api_secret }) = request_credentials.or(client_credentials)
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
                ureq_request = ureq_request.set("KEY", api_key);
                ureq_request = ureq_request.set("Timestamp", &timestamp.to_string());

                // Set x-gate-exptime header
                if let Some(exp_time_ms) = x_gate_exp_time {
                    ureq_request = ureq_request.set("x-gate-exptime", &exp_time_ms.to_string());
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

                ureq_request = ureq_request.set("SIGN", &signature);
            }
        }

        let raw_response = if payload.is_empty() {
            ureq_request.call() // Request without body
        } else {
            ureq_request.send_string(&payload) // Request with body
        };

        let response = match raw_response {
            Ok(response) => Ok(response),
            Err(UreqError::Status(_, response)) => Ok(response),
            Err(err) => Err(Error::Send(err)),
        }?;

        Ok(Response::from(response))
    }
}