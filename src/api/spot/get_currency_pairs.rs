use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving all supported spot currency pairs
pub struct GetCurrencyPairs {
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetCurrencyPairs {
    /// Creates a new GetCurrencyPairs request
    pub fn new() -> Self {
        Self { credentials: None }
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrencyPairs> for Request {
    fn from(request: GetCurrencyPairs) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: "/api/v4/spot/currency_pairs".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
