use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving all supported spot currencies
pub struct GetCurrencies {
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetCurrencies {
    /// Creates a new GetCurrencies request
    pub fn new() -> Self {
        Self { credentials: None }
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrencies> for Request {
    fn from(request: GetCurrencies) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: "/api/v4/spot/currencies".into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
