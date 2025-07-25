use crate::http::{Credentials, Method, request::Request};

/// Request for retrieving information about a specific currency
pub struct GetCurrency {
    /// Currency symbol to get information for
    pub currency: String,
    /// API credentials for authentication (optional for public data)
    pub credentials: Option<Credentials>,
}

impl GetCurrency {
    /// Creates a new GetCurrency request for the specified currency
    pub fn new(currency: &str) -> Self {
        Self {
            currency: currency.to_owned(),
            credentials: None,
        }
    }

    /// Sets the API credentials for authentication
    pub fn credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }
}

impl From<GetCurrency> for Request {
    fn from(request: GetCurrency) -> Request {
        let params = Vec::new();

        Request {
            method: Method::Get,
            path: format!("/api/v4/spot/currencies/{}", request.currency).into(),
            params,
            payload: "".to_string(),
            x_gate_exp_time: None,
            credentials: request.credentials,
            sign: false,
        }
    }
}
